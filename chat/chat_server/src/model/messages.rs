use super::ChatFile;
use crate::{agent::AgentVariant, AppError, AppState};
use chat_core::{Agent, AgentContext, AgentDecision, ChatType, Message};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::warn;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, IntoParams, ToSchema, Serialize, Deserialize)]
pub struct CreateMessage {
    pub content: String,
    #[serde(default)]
    pub files: Vec<String>,
}

#[derive(Debug, Clone, IntoParams, ToSchema, Serialize, Deserialize)]
pub struct ListMessages {
    #[serde(default)]
    pub last_id: Option<u64>,
    #[serde(default)]
    pub limit: u64,
}

#[allow(dead_code)]
impl AppState {
    pub async fn create_message(
        &self,
        input: CreateMessage,
        chat_id: u64,
        user_id: u64,
    ) -> Result<Message, AppError> {
        let base_dir = &self.config.server.base_url;
        if input.content.is_empty() && input.files.is_empty() {
            return Err(AppError::CreateMessageError(
                "Content or files must be provided".to_string(),
            ));
        }
        for s in &input.files {
            let file = ChatFile::from_str(s)?;
            if !file.path(base_dir).exists() {
                return Err(AppError::CreateMessageError(format!(
                    "Invalid chat file path: {}",
                    s
                )));
            }
        }

        // If we have agent, apply it and get the result
        let mut agents = self.list_agent(chat_id).await?;
        let decision = if let Some(agent) = agents.pop() {
            let agent: AgentVariant = agent.into();
            match agent {
                AgentVariant::Proxy(agent) => {
                    agent
                        .process(&input.content, &AgentContext::default())
                        .await?
                }
                _ => AgentDecision::None,
            }
        } else {
            AgentDecision::None
        };

        let modified_content = match decision {
            AgentDecision::Modify(ref content) => Some(content),
            _ => None,
        };

        let message: Message = sqlx::query_as(
            r#"
            INSERT INTO messages (chat_id, sender_id, content, modified_content, files)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, chat_id, sender_id, content, modified_content, files, created_at
            "#,
        )
        .bind(chat_id as i64)
        .bind(user_id as i64)
        .bind(input.content)
        .bind(modified_content)
        .bind(&input.files)
        .fetch_one(&self.pool)
        .await?;

        // If decision is reply(mean the user is chat with agent), create a new message
        if let AgentDecision::Reply(ref reply) = decision {
            let chat = self
                .get_chat_by_id(chat_id)
                .await?
                .expect("chat should exist");
            if chat.r#type != ChatType::Single {
                warn!(
                    "reply decision found in non single chat {}. reply: {}",
                    chat_id, reply
                );
            }
            let agent_user_id = chat
                .members
                .into_iter()
                .find(|id| id != &(user_id as i64))
                .expect("agent user id should exist");
            sqlx::query_as::<_, ()>(
                r#"
                INSERT INTO messages (chat_id, sender_id, content)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(chat_id as i64)
            .bind(agent_user_id)
            .bind(reply)
            .fetch_one(&self.pool)
            .await?;
        }
        Ok(message)
    }

    pub async fn list_message(
        &self,
        input: ListMessages,
        chat_id: u64,
    ) -> Result<Vec<Message>, AppError> {
        let last_id = input.last_id.unwrap_or(i64::MAX as _);
        let limit = match input.limit {
            0 => i64::MAX,
            1..=100 => input.limit as _,
            _ => 100,
        };
        let messages: Vec<Message> = sqlx::query_as(
            r#"
            SELECT id, chat_id, sender_id, content, modified_content, files, created_at
            FROM messages
            WHERE chat_id = $1 AND id < $2
            ORDER BY id DESC
            LIMIT $3
            "#,
        )
        .bind(chat_id as i64)
        .bind(last_id as i64)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serde_json::json;
    use wiremock::{matchers::path, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn create_message_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let _mock_server = mock_ollama_server().await;
        let input = CreateMessage {
            content: "hello".to_string(),
            files: vec![],
        };
        let message = state
            .create_message(input, 1, 1)
            .await
            .expect("create message filed");
        assert_eq!(message.content, "hello");

        //invalid files should fail
        let input = CreateMessage {
            content: "hello".to_string(),
            files: vec!["1".to_string()],
        };
        let err = state.create_message(input, 1, 1).await.unwrap_err();
        assert_eq!(err.to_string(), "Invalid chat file path: 1");

        //valid files should work
        let url = upload_dummy_file(&state)?;
        let input = CreateMessage {
            content: "hello".to_string(),
            files: vec![url],
        };
        let message = state
            .create_message(input, 1, 1)
            .await
            .expect("create message failed");
        assert_eq!(message.content, "hello");
        assert_eq!(message.files.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn list_messages_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = ListMessages {
            last_id: None,
            limit: 6,
        };

        let messages = state.list_message(input, 1).await?;
        println!("{:#?}", messages);
        assert_eq!(messages.len(), 6);

        let last_id = messages.last().expect("last message should exists").id;

        let input = ListMessages {
            last_id: Some(last_id as _),
            limit: 6,
        };
        let messages = state.list_message(input, 1).await?;
        println!("{:#?}", messages);
        assert_eq!(messages.len(), 4);

        Ok(())
    }

    fn upload_dummy_file(state: &AppState) -> Result<String> {
        let file = ChatFile::new(1, "test.txt", b"hello world");
        let path = file.path(&state.config.server.base_url);

        std::fs::create_dir_all(path.parent().expect("file path parent should exists"))?;
        std::fs::write(&path, b"hello world")?;

        Ok(file.url())
    }

    async fn mock_ollama_server() -> MockServer {
        let listener = std::net::TcpListener::bind("127.0.0.1:11434").unwrap();
        let mock_server = MockServer::builder().listener(listener).start().await;
        Mock::given(path("/api/chat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "model": "qwen3:0.6b",
                "created_at": "2024-01-01T00:00:00.000Z",
                "message": {
                    "role": "assistant",
                    "content": "Hello, world!"
                },
                "done": false,
                "total_duration": 1000,
                "load_duration": 1000,
                "prompt_eval_count": 1000,
                "prompt_eval_duration": 1000,
                "eval_count": 1000,
                "eval_duration": 1000
            })))
            .mount(&mock_server)
            .await;
        mock_server
    }
}

use crate::{AppError, AppState};
use chat_core::{AdapterType, AgentType, ChatAgent};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize)]
pub struct CreateAgent {
    pub name: String,
    pub r#type: AgentType,
    pub adapter: AdapterType,
    pub model: String,
    pub prompt: String,
    #[serde(default = "default_map")]
    pub args: serde_json::Value,
}

fn default_map() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[derive(Debug, Clone, Default, IntoParams, ToSchema, Serialize, Deserialize)]
pub struct UpdateAgent {
    pub id: u64,
    pub prompt: String,
    #[serde(default = "default_map")]
    pub args: serde_json::Value,
}

#[allow(dead_code)]
impl AppState {
    pub async fn create_agent(
        &self,
        input: CreateAgent,
        chat_id: u64,
    ) -> Result<ChatAgent, AppError> {
        if self.agent_name_exists(chat_id, &input.name).await? {
            info!("Agent name {} already exists", input.name);
            return Err(AppError::CreateChatError(format!(
                "Agent name {} already exists",
                input.name
            )));
        }

        let chat = sqlx::query_as(
            r#"
            INSERT INTO chat_agents (chat_id, name, type, adapter, model, prompt, args)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
                "#,
        )
        .bind(chat_id as i64)
        .bind(input.name)
        .bind(input.r#type)
        .bind(input.adapter)
        .bind(input.model)
        .bind(input.prompt)
        .bind(input.args)
        .fetch_one(&self.pool)
        .await?;
        Ok(chat)
    }

    pub async fn list_agent(&self, chat_id: u64) -> Result<Vec<ChatAgent>, AppError> {
        let chats = sqlx::query_as(
            r#"
            SELECT *
            FROM chat_agents
            WHERE chat_id = $1
            ORDER BY id ASC
                "#,
        )
        .bind(chat_id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(chats)
    }

    pub async fn get_agent_by_id(
        &self,
        chat_id: u64,
        id: u64,
    ) -> Result<Option<ChatAgent>, AppError> {
        let chat = sqlx::query_as(
            r#"
            SELECT *
            FROM chat_agents
            WHERE chat_id = $1 AND id = $2
                "#,
        )
        .bind(chat_id as i64)
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(chat)
    }

    pub async fn update_agent(&self, input: UpdateAgent) -> Result<ChatAgent, AppError> {
        if !self.agent_id_exists(input.id).await? {
            return Err(AppError::UpdateAgentError(format!(
                "Agent id {} does not exist",
                input.id
            )));
        }

        let agent = sqlx::query_as(
            r#"
            UPDATE chat_agents
            SET prompt = $1, args = $2
            WHERE id = $3
            RETURNING *
                "#,
        )
        .bind(input.prompt)
        .bind(input.args)
        .bind(input.id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(agent)
    }

    pub async fn delete_agent_by_id(&self, id: u64) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM chat_agents
            WHERE id = $1
                "#,
        )
        .bind(id as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn agent_name_exists(&self, chat_id: u64, name: &str) -> Result<bool, AppError> {
        let exists = sqlx::query(
            r#"
            SELECT 1
            FROM chat_agents
            WHERE name = $1 AND chat_id = $2
                "#,
        )
        .bind(name)
        .bind(chat_id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(exists.is_some())
    }

    async fn agent_id_exists(&self, id: u64) -> Result<bool, AppError> {
        let exists = sqlx::query(
            r#"
            SELECT 1
            FROM chat_agents
            WHERE id = $1
                "#,
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(exists.is_some())
    }
}

impl CreateAgent {
    pub fn new(
        name: impl Into<String>,
        r#type: AgentType,
        adapter: AdapterType,
        model: impl Into<String>,
        prompt: impl Into<String>,
        args: impl Serialize,
    ) -> Self {
        Self {
            name: name.into(),
            r#type,
            adapter,
            model: model.into(),
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}

impl UpdateAgent {
    pub fn new(id: u64, prompt: impl Into<String>, args: impl Serialize) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use anyhow::Result;
    use chat_core::AgentType;

    #[tokio::test]
    async fn create_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateAgent::new(
            "test",
            AgentType::Reply,
            AdapterType::Ollama,
            "llama3.2",
            "You are a helpful assistant",
            HashMap::<String, String>::new(),
        );
        let agent = state
            .create_agent(input, 1)
            .await
            .expect("create agent failed");
        assert_eq!(agent.chat_id, 1);
        assert_eq!(agent.name, "test");
        assert_eq!(agent.r#type, AgentType::Reply);
        assert_eq!(agent.adapter, AdapterType::Ollama);
        assert_eq!(agent.model, "llama3.2");
        assert_eq!(agent.prompt, "You are a helpful assistant");
        assert_eq!(agent.args, serde_json::json!({}));

        Ok(())
    }

    #[tokio::test]
    async fn get_agent_by_id_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let agent = state
            .get_agent_by_id(1, 1)
            .await
            .expect("get agent by id failed")
            .unwrap();
        assert_eq!(agent.id, 1);
        assert_eq!(agent.name, "translation");
        assert_eq!(agent.chat_id, 1);
        assert_eq!(agent.r#type, AgentType::Proxy);
        assert_eq!(agent.adapter, AdapterType::Ollama);
        assert_eq!(agent.model, "qwen3:0.6b");
        assert_eq!(agent.prompt, "If language is Chinese, translate to English, if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content: ");
        assert_eq!(agent.args, serde_json::json!({}));

        Ok(())
    }

    #[tokio::test]
    async fn list_agents_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let agents = state.list_agent(1).await.expect("fetch all agents failed");

        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "translation");
        assert_eq!(agents[0].r#type, AgentType::Proxy);
        assert_eq!(agents[0].adapter, AdapterType::Ollama);
        assert_eq!(agents[0].model, "qwen3:0.6b");
        assert_eq!(agents[0].prompt, "If language is Chinese, translate to English, if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content: ");
        assert_eq!(agents[0].args, serde_json::json!({}));

        Ok(())
    }

    #[tokio::test]
    async fn delete_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        state
            .delete_agent_by_id(1)
            .await
            .expect("delete agent failed");
        let agent = state
            .get_agent_by_id(1, 1)
            .await
            .expect("get agent by id failed");
        assert!(agent.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn update_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // create an agent
        let input = CreateAgent::new(
            "test",
            AgentType::Proxy,
            AdapterType::Ollama,
            "llama3.2",
            "You are a helpful assistant",
            HashMap::<String, String>::new(),
        );
        let agent = state
            .create_agent(input, 1)
            .await
            .expect("create agent failed");
        // update the agent
        let input = UpdateAgent::new(
            agent.id as u64,
            "Can you tell me the weather in Tokyo?",
            HashMap::<String, String>::new(),
        );
        let agent = state
            .update_agent(input)
            .await
            .expect("update agent failed");
        assert_eq!(agent.prompt, "Can you tell me the weather in Tokyo?");
        assert_eq!(agent.args, serde_json::json!({}));
        Ok(())
    }
}

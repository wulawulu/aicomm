use crate::{AppError, AppState};
use chat_core::{Chat, ChatType};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct CreateChat {
    pub name: Option<String>,
    pub members: Vec<i64>,
    pub public: bool,
}

#[derive(Debug, Clone, IntoParams, ToSchema, Serialize, Deserialize)]
pub struct UpdateChat {
    pub name: Option<String>,
    pub members: Option<Vec<i64>>,
    pub chat_type: Option<ChatType>,
}

#[allow(dead_code)]
impl AppState {
    pub async fn create_chat(
        &self,
        input: CreateChat,
        user_id: u64,
        ws_id: u64,
    ) -> Result<Chat, AppError> {
        let len = input.members.len();
        if len < 2 {
            return Err(AppError::CreateChatError(
                "Chat must have at least 2 members".to_string(),
            ));
        }

        if !input.members.contains(&(user_id as i64)) {
            return Err(AppError::CreateChatError(
                "You must be a member of the chat".to_string(),
            ));
        }

        if len > 8 && input.name.is_none() {
            return Err(AppError::CreateChatError(
                "Group chat with more than 8 members must have a name".to_string(),
            ));
        }

        let user = self.fetch_chat_user_by_ids(&input.members).await?;
        if user.len() != len {
            return Err(AppError::CreateChatError(
                "Some members do not exist".to_string(),
            ));
        }

        let chat_type = match (&input.name, len) {
            (None, 2) => ChatType::Single,
            (None, _) => ChatType::Group,
            (Some(_), _) => {
                if input.public {
                    ChatType::PublicChannel
                } else {
                    ChatType::PrivateChannel
                }
            }
        };

        let chat = sqlx::query_as(
            r#"
            INSERT INTO chats (ws_id, name, type, members)
            VALUES ($1, $2, $3, $4)
            RETURNING id, ws_id, name, type, members,created_at
                "#,
        )
        .bind(ws_id as i64)
        .bind(input.name)
        .bind(chat_type)
        .bind(&input.members)
        .fetch_one(&self.pool)
        .await?;
        Ok(chat)
    }

    pub async fn fetch_chats(&self, user_id: u64, ws_id: u64) -> Result<Vec<Chat>, AppError> {
        let chats = sqlx::query_as(
            r#"
            SELECT id, ws_id, name, type, members, created_at
            FROM chats
            WHERE ws_id = $1 AND $2 = ANY(members)
                "#,
        )
        .bind(ws_id as i64)
        .bind(user_id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(chats)
    }

    pub async fn get_chat_by_id(&self, id: u64) -> Result<Option<Chat>, AppError> {
        let chat = sqlx::query_as(
            r#"
            SELECT id, ws_id, name, type, members, created_at
            FROM chats
            WHERE id = $1
                "#,
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(chat)
    }

    pub async fn update_chat_by_id(&self, id: u64, input: UpdateChat) -> Result<Chat, AppError> {
        let chat = self.get_chat_by_id(id).await?;
        let chat = match chat {
            None => return Err(AppError::NotFound(format!("chat id {}", id))),
            Some(chat) => chat,
        };

        let mut name = chat.name;
        if let Some(new_name) = input.name {
            name = Some(new_name);
        }

        let mut members = chat.members;
        if let Some(new_members) = input.members {
            if chat.r#type.eq(&ChatType::Single) {
                return Err(AppError::UpdateChatError(
                    "Cannot update members of a single chat".to_string(),
                ));
            }
            let user = self.fetch_chat_user_by_ids(&new_members).await?;
            if user.len() != new_members.len() {
                return Err(AppError::UpdateChatError(
                    "Some members do not exist".to_string(),
                ));
            }
            members = new_members;
        }

        let mut r#type = chat.r#type;
        if let Some(new_type) = input.chat_type {
            if r#type == ChatType::Single || r#type == ChatType::Group {
                return Err(AppError::UpdateChatError(
                    "Cannot update type of a single or group chat".to_string(),
                ));
            }
            r#type = new_type;
        }

        let chat = sqlx::query_as(
            r#"
            UPDATE chats
            SET name = $1, members = $2, type = $3
            WHERE id = $4
            RETURNING id, ws_id, name, type, members, created_at
                "#,
        )
        .bind(name)
        .bind(&members)
        .bind(r#type)
        .bind(id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(chat)
    }

    pub async fn delete_chat_by_id(&self, id: u64) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM chats
            WHERE id = $1
                "#,
        )
        .bind(id as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn is_chat_member(&self, chat_id: u64, user_id: u64) -> Result<bool, AppError> {
        let is_member = sqlx::query(
            r#"
            SELECT 1
            FROM chats
            WHERE id = $1 AND $2 = ANY(members)
                "#,
        )
        .bind(chat_id as i64)
        .bind(user_id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(is_member.is_some())
    }
}

#[cfg(test)]
impl CreateChat {
    pub fn new(name: &str, members: &[i64], public: bool) -> Self {
        let name = if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        };
        Self {
            name,
            members: members.to_vec(),
            public,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chat_core::ChatType;

    #[tokio::test]
    async fn create_single_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateChat::new("", &[1, 2], false);
        let chat = state
            .create_chat(input, 1, 1)
            .await
            .expect("create chat failed");
        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 2);
        assert_eq!(chat.r#type, ChatType::Single);

        Ok(())
    }

    #[tokio::test]
    async fn create_public_named_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateChat::new("test", &[1, 2, 3], true);
        let chat = state
            .create_chat(input, 1, 1)
            .await
            .expect("create chat failed");
        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 3);
        assert_eq!(chat.r#type, ChatType::PublicChannel);

        Ok(())
    }

    #[tokio::test]
    async fn chat_get_by_id_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let chat = state
            .get_chat_by_id(1)
            .await
            .expect("get chat by id failed")
            .unwrap();
        assert_eq!(chat.id, 1);
        assert_eq!(chat.name.expect("chat name"), "general");
        assert_eq!(chat.ws_id, 1);
        assert_eq!(chat.members.len(), 5);

        Ok(())
    }

    #[tokio::test]
    async fn chat_fetch_all_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let chats = state
            .fetch_chats(1, 1)
            .await
            .expect("fetch all chats failed");

        assert_eq!(chats.len(), 4);

        Ok(())
    }

    #[tokio::test]
    async fn delete_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        state
            .delete_chat_by_id(1)
            .await
            .expect("delete chat failed");
        let chat = state
            .get_chat_by_id(1)
            .await
            .expect("get chat by id failed");
        assert!(chat.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn update_chat_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = UpdateChat {
            name: Some("new name".to_string()),
            members: Some(vec![1, 2, 3]),
            chat_type: None,
        };
        let chat = state
            .update_chat_by_id(1, input)
            .await
            .expect("update chat failed");
        assert_eq!(chat.name.expect("chat name"), "new name");
        assert_eq!(chat.members.len(), 3);
        assert_eq!(chat.r#type, ChatType::PublicChannel);

        Ok(())
    }

    #[tokio::test]
    async fn is_chat_member_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let is_member = state.is_chat_member(1, 1).await.expect("is member failed");
        assert!(is_member);

        // user 6 doesn't exist
        let is_member = state.is_chat_member(1, 6).await.expect("is member failed");
        assert!(!is_member);

        // chat 10 doesn't exist
        let is_member = state.is_chat_member(10, 1).await.expect("is member failed");
        assert!(!is_member);

        // user 4 is not a member of chat 2
        let is_member = state.is_chat_member(2, 4).await.expect("is member failed");
        assert!(!is_member);

        Ok(())
    }
}

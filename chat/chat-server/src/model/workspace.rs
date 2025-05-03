use crate::{AppError, AppState};
use chat_core::{ChatUser, Workspace};

impl AppState {
    pub async fn create_workspace(&self, name: &str, user_id: u64) -> Result<Workspace, AppError> {
        let row = sqlx::query_as(
            r#"
            INSERT INTO workspaces(name,owner_id)
            VALUES($1,$2)
            RETURNING id,name,owner_id,created_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_workspace_by_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"
            SELECT id,name,owner_id,created_at
            FROM workspaces
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(ws)
    }

    #[allow(dead_code)]
    pub async fn find_workspace_by_id(&self, id: u64) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"
            SELECT id,name,owner_id,created_at
            FROM workspaces
            WHERE id = $1
            "#,
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(ws)
    }

    #[allow(dead_code)]
    pub async fn fetch_chat_users(&self, id: u64) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
            SELECT id, fullname, email
            FROM users
            WHERE ws_id = $1 ORDER BY id
            "#,
        )
        .bind(id as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update_workspace_owner(
        &self,
        id: u64,
        owner_id: u64,
    ) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(
            r#"
            UPDATE workspaces
            SET owner_id = $1
            WHERE id = $2 AND (SELECT ws_id FROM users WHERE id = $1) = $2
            RETURNING id,name,owner_id,created_at
            "#,
        )
        .bind(owner_id as i64)
        .bind(id as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(ws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::CreateUser;
    use anyhow::Result;

    ///TODO: Add more tests
    #[tokio::test]
    async fn workspace_should_create_and_set_owner() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state
            .create_workspace("test", 0)
            .await
            .expect("cannot create workspace");
        assert_eq!(ws.name, "test");

        let user = CreateUser::new(&ws.name, "wu", "wu@github.con", "123456");
        let user = state.create_user(&user).await.expect("cannot create user");

        assert_eq!(user.ws_id, ws.id);

        let Some(ws) = state
            .find_workspace_by_id(ws.id as u64)
            .await
            .expect("cannot find workspace")
        else {
            panic!("cannot find workspace")
        };
        assert_eq!(ws.owner_id, user.id);

        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_find_by_name() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state
            .find_workspace_by_name("foo")
            .await
            .expect("cannot find workspace");
        assert_eq!(ws.expect("cannot find by name").name, "foo");
        Ok(())
    }

    #[tokio::test]
    async fn workspace_should_fetch_all_chat_users() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let users = state
            .fetch_chat_users(1)
            .await
            .expect("cannot fetch all chat users");
        assert_eq!(users.len(), 5);

        Ok(())
    }
}

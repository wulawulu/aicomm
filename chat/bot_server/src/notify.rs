use chat_core::Message;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;
use swiftide::{
    integrations::pgvector::PgVector,
    traits::{EmbeddingModel, SimplePrompt},
};
use tracing::info;

use crate::query;

#[derive(Debug)]
pub struct Notification {
    bot_id: i64,
    event: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageCreated {
    message: Message,
    members: HashSet<i64>,
}

impl Notification {
    pub fn load(r#type: &str, payload: &str, bots: &HashSet<i64>) -> Option<Self> {
        match r#type {
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(payload).ok()?;
                let mut members = payload.members;
                members.remove(&payload.message.sender_id);

                // only process if it's a direct message
                if members.len() == 1 {
                    let bot_id = members.iter().next().unwrap();
                    if bots.contains(bot_id) {
                        return Some(Self {
                            bot_id: *bot_id,
                            event: payload.message,
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub async fn process(
        self,
        pool: PgPool,
        store: PgVector,
        client: impl SimplePrompt + Clone + 'static,
        fastembed: impl EmbeddingModel + Clone + 'static,
    ) -> anyhow::Result<()> {
        let answer = query(self.event.content, store, client, fastembed).await?;
        info!("Got answer : {}. Writing to db...", answer);
        let _: (i64,) = sqlx::query_as(
            r#"
          INSERT INTO messages (chat_id, sender_id, content)
          VALUES ($1, $2, $3)
          RETURNING id
          "#,
        )
        .bind(self.event.chat_id)
        .bind(self.bot_id)
        .bind(answer)
        .fetch_one(&pool)
        .await?;
        Ok(())
    }
}

use crate::AppState;
use chat_core::{Chat, Message};
use jwt_simple::reexports::serde_json;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;
use std::collections::HashSet;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{info, warn};

#[derive(Debug, Deserialize)]
struct ChatUpdated {
    op: String,
    old: Option<Chat>,
    new: Option<Chat>,
}

#[derive(Debug, Deserialize)]
struct ChatMessageCreated {
    message: Message,
    members: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    NewChat(Chat),
    AddToChat(Chat),
    ChatNameUpdate(Chat),
    RemoveFromChat(Chat),
    NewMessage(Message),
}

#[derive(Debug)]
pub struct Notification {
    affect_users: HashSet<u64>,
    event: Arc<AppEvent>,
}

impl Notification {
    fn load(channel: &str, payload: &str) -> anyhow::Result<Self> {
        match channel {
            "chat_updated" => {
                let data: ChatUpdated = serde_json::from_str(payload)?;
                info!("ChatUpdated: {:?}", data);
                let affected_members =
                    get_affected_chat_user_ids(data.old.as_ref(), data.new.as_ref());
                let event = match data.op.as_str() {
                    "INSERT" => AppEvent::NewChat(data.new.expect("new should exist")),
                    "UPDATE" => match (data.old, data.new) {
                        (Some(old), Some(new)) => {
                            if old.members.eq(&new.members) {
                                AppEvent::ChatNameUpdate(new)
                            } else {
                                AppEvent::AddToChat(new)
                            }
                        }
                        (_, _) => {
                            return Err(anyhow::anyhow!("update should have both old and new"))
                        }
                    },
                    "DELETE" => AppEvent::RemoveFromChat(data.old.expect("old should exist")),
                    _ => return Err(anyhow::anyhow!("Invalid operation: {}", data.op)),
                };
                Ok(Self {
                    affect_users: affected_members,
                    event: Arc::new(event),
                })
            }
            "chat_message_created" => {
                let data: ChatMessageCreated =
                    serde_json::from_str(payload).expect("failed to parse");
                Ok(Self {
                    affect_users: data.members.into_iter().map(|v| v as u64).collect(),
                    event: Arc::new(AppEvent::NewMessage(data.message)),
                })
            }
            _ => Err(anyhow::anyhow!("Invalid channel: {}", channel)),
        }
    }
}

fn get_affected_chat_user_ids(old: Option<&Chat>, new: Option<&Chat>) -> HashSet<u64> {
    match (old, new) {
        (Some(old), Some(new)) => {
            // diff old/new members, if identical, return empty set,otherwise return the union
            let old_members: HashSet<_> = old.members.iter().map(|v| *v as u64).collect();
            let new_members: HashSet<_> = new.members.iter().map(|v| *v as u64).collect();
            if old_members == new_members {
                new_members
            } else {
                old_members.union(&new_members).copied().collect()
            }
        }
        (Some(old), None) => old.members.iter().map(|v| *v as u64).collect(),
        (None, Some(new)) => new.members.iter().map(|v| *v as u64).collect(),
        _ => HashSet::new(),
    }
}

pub async fn setup_pg_listener(state: AppState) -> anyhow::Result<()> {
    let mut listener = PgListener::connect(&state.config.server.db_url).await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;

    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(Ok(notif)) = stream.next().await {
            info!("Received notification: {:?}", notif);
            let notification = Notification::load(notif.channel(), notif.payload())?;
            info!("Notification: {:?}", notification);
            let users = &state.users;
            for user_id in notification.affect_users {
                if let Some(entry) = users.get(&user_id) {
                    info!("Sending notification to user {}", user_id);
                    if let Err(e) = entry.value().send(notification.event.clone()) {
                        warn!("Failed to send notification to user {}: {}", user_id, e);
                    }
                }
            }
        }
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

use serde::{Deserialize, Serialize};

mod chat;
mod file;
mod messages;
mod user;
mod workspace;

pub use chat::{CreateChat, UpdateChat};
pub use messages::{CreateMessage, ListMessages};
pub use user::{CreateUser, SigninUser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,
    pub ext: String,
    pub hash: String,
}

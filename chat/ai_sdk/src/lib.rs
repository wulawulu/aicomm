mod adapters;

pub use adapters::*;

#[derive(Debug, Clone)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[allow(async_fn_in_trait)]
pub trait AiService {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String>;
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
            }
        )
    }
}

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{AiService, Message};

pub struct CloudflareAdapter {
    account_id: String,
    api_token: String,
    client: Client,
}

#[derive(Serialize)]
pub struct CloudflareChatCompletionRequest {
    pub messages: Vec<CloudflareMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct CloudflareMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CloudflareChatCompletionResponse {
    pub result: ResponseResult,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct ResponseResult {
    pub response: String,
    pub usage: CloudflareUsage,
}

#[derive(Deserialize)]
pub struct CloudflareUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl CloudflareAdapter {
    pub fn new(account_id: impl Into<String>, api_token: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            api_token: api_token.into(),
            client: Client::new(),
        }
    }
}

impl AiService for CloudflareAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = CloudflareChatCompletionRequest {
            messages: messages.iter().map(|m| m.into()).collect(),
        };
        let url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/@cf/meta/llama-3-8b-instruct",
            self.account_id
        );
        let response = self
            .client
            .post(url)
            .json(&request)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?;
        let response: CloudflareChatCompletionResponse = response.json().await?;
        Ok(response.result.response)
    }
}

impl From<Message> for CloudflareMessage {
    fn from(message: Message) -> Self {
        CloudflareMessage {
            role: message.role.to_string(),
            content: message.content,
        }
    }
}

impl From<&Message> for CloudflareMessage {
    fn from(message: &Message) -> Self {
        CloudflareMessage {
            role: message.role.to_string(),
            content: message.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::Role;

    #[ignore]
    #[tokio::test]
    async fn cloudflare_complete_should_work() {
        let account_id = env::var("CLOUDFLARE_ACCOUNT_ID").unwrap();
        let api_token = env::var("CLOUDFLARE_API_TOKEN").unwrap();
        let adapter = CloudflareAdapter::new(account_id, api_token);
        let messages = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
        }];
        let response = adapter.complete(&messages).await.unwrap();
        println!("response: {}", response);
    }
}

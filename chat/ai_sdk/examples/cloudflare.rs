use ai_sdk::*;
use std::env;

#[tokio::main]
async fn main() {
    let account_id = env::var("CLOUDFLARE_ACCOUNT_ID").unwrap();
    let api_token = env::var("CLOUDFLARE_API_TOKEN").unwrap();
    let adapter = CloudflareAdapter::new(account_id, api_token);
    let messages = vec![Message {
        role: Role::User,
        content: "世界上最长的河流是什么？".to_string(),
    }];
    let response = adapter.complete(&messages).await.unwrap();
    println!("response: {}", response);
}

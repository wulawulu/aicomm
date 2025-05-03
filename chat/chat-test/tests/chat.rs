use anyhow::Result;
use chat_core::{Chat, ChatType, Message};
use futures::StreamExt;
use notify_server::{AppConfig, get_router};
use reqwest::{
    Client, StatusCode,
    header::{AUTHORIZATION, CONTENT_TYPE},
    multipart::{Form, Part},
};
use reqwest_eventsource::{Event, EventSource};
use serde::Deserialize;
use serde_json::json;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, time::sleep};

struct ChatServer {
    addr: SocketAddr,
    token: String,
    client: Client,
}

struct NotifyServer;

#[derive(Debug, Deserialize)]
struct AuthToken {
    token: String,
}
const WILD_ADDR: &str = "0.0.0.0:0";

#[tokio::test]
async fn test_chat_server() -> Result<()> {
    let (tdb, state) = chat_server::AppState::new_for_test().await?;
    let chat_server = ChatServer::new(state).await?;
    let db_url = tdb.url();
    NotifyServer::new(&db_url, &chat_server.token).await?;

    let chat = chat_server.create_chat().await?;
    let _msg = chat_server.create_message(chat.id as u64).await?;
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

impl ChatServer {
    async fn new(state: chat_server::AppState) -> Result<Self> {
        let app = chat_server::get_router(state).await?;
        let listener = TcpListener::bind(WILD_ADDR).await?;
        let addr = listener.local_addr()?;

        tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap()
        });

        let mut ret = ChatServer {
            addr,
            token: "".to_string(),
            client: Client::new(),
        };
        let token = ret.signin().await?;
        ret.token = token;
        Ok(ret)
    }

    async fn signin(&self) -> Result<String> {
        let res = self
            .client
            .post(format!("http://{}/api/signin", self.addr))
            .header(CONTENT_TYPE, "application/json")
            .body(r#"{"email": "wu@github.org","password": "123456"}"#)
            .send()
            .await?;
        assert_eq!(res.status(), StatusCode::OK);
        let result: AuthToken = res.json().await?;
        Ok(result.token)
    }

    async fn create_chat(&self) -> Result<Chat> {
        let res = self
            .client
            .post(format!("http://{}/api/chats", self.addr))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .body(r#"{"name": "github","members": [1, 2],"public": false}"#)
            .send()
            .await?;

        assert_eq!(res.status(), StatusCode::CREATED);

        let chat: Chat = res.json().await?;
        assert_eq!(chat.name.as_ref().unwrap(), "github");
        assert_eq!(chat.members, vec![1, 2]);
        assert_eq!(chat.r#type, ChatType::PrivateChannel);

        Ok(chat)
    }

    async fn create_message(&self, chat_id: u64) -> Result<Message> {
        let data = include_bytes!("../Cargo.toml");
        let files = Part::bytes(data)
            .file_name("Cargo.toml")
            .mime_str("text/plain")?;
        let form = Form::new().part("file", files);

        let res = self
            .client
            .post(format!("http://{}/api/upload", self.addr))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .multipart(form)
            .send()
            .await?;
        assert_eq!(res.status(), StatusCode::OK);
        let ret: Vec<String> = res.json().await?;
        println!("{:?}", ret);

        let body = serde_json::to_string(&json!({
            "content":"hello",
            "files":ret
        }))?;
        let res = self
            .client
            .post(format!(
                "http://{}/api/chats/{}/messages",
                self.addr, chat_id
            ))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?;
        assert_eq!(res.status(), StatusCode::CREATED);
        let message: Message = res.json().await?;
        assert_eq!(message.content, "hello");
        assert_eq!(message.files, ret);
        assert_eq!(message.sender_id, 1);
        assert_eq!(message.chat_id, chat_id as i64);
        Ok(message)
    }
}

impl NotifyServer {
    async fn new(db_url: &str, token: &str) -> Result<Self> {
        let mut config = AppConfig::load()?;
        config.server.db_url = db_url.to_string();
        let app = get_router(config).await?;
        let listener = TcpListener::bind(&WILD_ADDR).await?;
        let addr = listener.local_addr()?;
        tokio::spawn(async move {
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        });

        let mut es = EventSource::get(format!("http://{}/events?token={}", addr, token));

        tokio::spawn(async move {
            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => println!("Connection Open!"),
                    Ok(Event::Message(message)) => match message.event.as_str() {
                        "NewChat" => {
                            let chat: Chat = serde_json::from_str(&message.data).unwrap();
                            assert_eq!(chat.name.as_ref().unwrap(), "github");
                            assert_eq!(chat.members, vec![1, 2]);
                            assert_eq!(chat.r#type, ChatType::PrivateChannel);
                        }
                        "NewMessage" => {
                            let msg: Message = serde_json::from_str(&message.data).unwrap();
                            assert_eq!(msg.content, "hello");
                            assert_eq!(msg.files.len(), 1);
                            assert_eq!(msg.sender_id, 1);
                        }
                        _ => {
                            panic!("Unexpected event received: {:?}", message);
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        es.close();
                    }
                }
            }
        });

        Ok(Self)
    }
}

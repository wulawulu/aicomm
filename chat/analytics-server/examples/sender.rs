use anyhow::{Context, Result};
use chat_core::pb::{
    AnalyticsEvent, AppExitEvent, EventContext, GeoLocation, SystemInfo,
    analytics_event::EventType, app_exit_event::ExitCode,
};
use prost::Message;
#[tokio::main]
async fn main() -> Result<()> {
    let mut context = EventContext {
        client_id: "client_123".to_string(),
        user_id: "user_123".to_string(),
        app_version: "1.0.0".to_string(),
        client_ts: chrono::Utc::now().timestamp_millis(),
        ..Default::default()
    };
    // this should be overwritten by server
    context.server_ts = chrono::Utc::now().timestamp_millis();
    context.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".to_string();
    context.ip = "127.0.0.1".to_string();
    context.system = Some(SystemInfo {
        os: "macos".to_string(),
        arch: "x64".to_string(),
        locale: "en-US".to_string(),
        timezone: "Asia/Shanghai".to_string(),
    });

    // this should be overwritten by server
    context.geo = Some(GeoLocation {
        country: "China".to_string(),
        region: "Shanghai".to_string(),
        city: "Shanghai".to_string(),
    });

    let exit = AppExitEvent {
        exit_code: ExitCode::Success.into(),
    };
    let event = AnalyticsEvent {
        context: Some(context),
        event_type: Some(EventType::AppExit(exit)),
    };
    println!("{:?}", event);
    let client = reqwest::Client::new();
    let data = Message::encode_to_vec(&event);
    // write data to "../../fixtures/event.bin"
    std::fs::write("../../fixtures/event.bin", &data).context("write event to file")?;
    // load data from "../../fixtures/event.bin"
    let data1 = std::fs::read("../../fixtures/event.bin").context("read event from file")?;
    // parse data1 to event
    let event1 = AnalyticsEvent::decode(data1.as_slice())?;
    println!("{:?}", event1);
    let res = client
        .post("http://127.0.0.1:6690/api/event")
        .header("content-type", "application/protobuf")
        .header("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJjdXN0b20iOiJ7XCJpZFwiOjEsXCJ3c0lkXCI6MSxcIndzTmFtZVwiOlwiYWNtZVwiLFwiZnVsbG5hbWVcIjpcIlR5ciBDaGVuXCIsXCJlbWFpbFwiOlwidGNoZW5AYWNtZS5vcmdcIixcImlzQm90XCI6ZmFsc2UsXCJjcmVhdGVkQXRcIjpcIjIwMjUtMDUtMTZUMTI6NTA6MzIuMDI1NzE3WlwifSIsImlzcyI6ImNoYXQtc2VydmVyIiwiYXVkIjoiY2hhdF93ZWIifQ.8fegMwIGaJe10iyX9HKJE45gU3yOplTik5cwnVUiW4CVuxFXI5IEqXRFOLyuzhOGnvnK5FIm52HA1Qqd1NT9Cw")
        .body(data)
        .send()
        .await?;
    println!("Server returned {:?}", res.status());
    let body = res.text().await?;
    println!("Server returned body {:?}", body);
    Ok(())
}

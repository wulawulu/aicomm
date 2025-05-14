mod config;
mod notify;
mod query;

pub use config::AppConfig;
pub const VECTOR_SIZE: i32 = 384;
use notify::Notification;
pub use query::query;

use sqlx::{
    PgPool,
    postgres::{PgListener, PgPoolOptions},
};
use std::collections::HashSet;
use swiftide::{
    indexing::{EmbeddedField, transformers::metadata_qa_text::NAME as METADATA_QA_TEXT_NAME},
    integrations::{self, pgvector::PgVector},
};
use tokio_stream::StreamExt;
use tracing::info;

pub async fn setup_pg_listener(config: &AppConfig) -> anyhow::Result<()> {
    let db_url = &config.server.db_url;
    let mut listener = PgListener::connect(db_url).await?;
    listener.listen("chat_message_created").await?;
    info!("Listening for chat_message_created");

    let pool = PgPoolOptions::new().connect(db_url).await?;
    let bots = get_bots(&pool).await?;

    let store = PgVector::builder()
        .db_url(db_url)
        .vector_size(VECTOR_SIZE)
        .with_vector(EmbeddedField::Combined)
        .with_metadata(METADATA_QA_TEXT_NAME)
        .table_name("swiftide".to_string())
        .build()
        .unwrap();

    let fastembed = integrations::fastembed::FastEmbed::try_default()?;
    let client = integrations::ollama::Ollama::default()
        .with_default_prompt_model("llama3.2")
        .to_owned();

    let mut stream = listener.into_stream();

    while let Some(Ok(notif)) = stream.next().await {
        info!("Received notification: {:?}", notif);
        let notification = Notification::load(notif.channel(), notif.payload(), &bots);
        if let Some(notification) = notification {
            let pool = pool.clone();
            let store = store.clone();
            let client = client.clone();
            let embed = fastembed.clone();
            tokio::spawn(async move { notification.process(pool, store, client, embed).await });
        }
    }

    Ok(())
}

async fn get_bots(pool: &PgPool) -> anyhow::Result<HashSet<i64>> {
    let bots: Vec<(i64,)> = sqlx::query_as(r#"SELECT id FROM users WHERE is_bot = TRUE"#)
        .fetch_all(pool)
        .await?;
    Ok(bots.into_iter().map(|b| b.0).collect())
}

use anyhow::Result;
use async_openai::config::OpenAIConfig;
use bot_server::{AppConfig, VECTOR_SIZE};
use swiftide::{
    indexing::{
        self, EmbeddedField,
        loaders::FileLoader,
        transformers::{
            ChunkCode, Embed, MetadataQACode, metadata_qa_code::NAME as METADATA_QA_CODE_NAME,
        },
    },
    integrations::{self, pgvector::PgVector},
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let db_url = &config.server.db_url;

    let fastembed = integrations::fastembed::FastEmbed::try_default()?;
    let client = integrations::openai::OpenAIBuilder::default()
        .client(async_openai::Client::with_config(
            OpenAIConfig::default()
                .with_api_base("https://api.deepseek.com")
                .to_owned(),
        ))
        .default_prompt_model("deepseek-chat")
        .build()?;
    let store = PgVector::builder()
        .db_url(db_url)
        .vector_size(VECTOR_SIZE)
        .with_vector(EmbeddedField::Combined)
        .with_metadata(METADATA_QA_CODE_NAME)
        .table_name("swiftide".to_string())
        .build()
        .unwrap();

    indexing::Pipeline::from_loader(FileLoader::new(".").with_extensions(&["rs"]))
        .then_chunk(ChunkCode::try_for_language_and_chunk_size(
            "rust",
            10..2048,
        )?)
        .then(MetadataQACode::new(client.clone()).with_concurrency(5))
        .then_in_batch(Embed::new(fastembed))
        .then_store_with(store)
        .run()
        .await?;
    Ok(())
}

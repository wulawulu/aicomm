use swiftide::{
    integrations::pgvector::PgVector,
    query::{self, answers, query_transformers, response_transformers},
    traits::{EmbeddingModel, SimplePrompt},
};
use tracing::info;

pub async fn query(
    question: impl Into<String>,
    store: PgVector,
    client: impl SimplePrompt + Clone + 'static,
    embed: impl EmbeddingModel + Clone + 'static,
) -> anyhow::Result<String> {
    let question = question.into();
    let pipeline = query::Pipeline::default()
        .then_transform_query(query_transformers::GenerateSubquestions::from_client(
            client.clone(),
        ))
        .then_transform_query(query_transformers::Embed::from_client(embed.clone()))
        .then_retrieve(store)
        .then_transform_response(response_transformers::Summary::from_client(client.clone()))
        .then_answer(answers::Simple::from_client(client.clone()));
    info!("Processing query: {:?}", question);
    let ret = pipeline.query(question).await?;
    let answer = ret.answer();
    Ok(answer.to_owned())
}

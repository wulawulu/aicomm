use axum::{
    extract::State,
    http::{StatusCode, request::Parts},
    response::IntoResponse,
};
use chat_core::pb::AnalyticsEvent;
use tracing::info;

use crate::{
    AppError, AppState, ErrorOutput,
    event::AnalyticsEventRow,
    extractors::{Geo, Protobuf},
};

#[utoipa::path(
    post,
    path = "/api/event",
    responses(
        (status = 201, description = "Event created"),
        (status = 400, description = "Invalid event", body = ErrorOutput),
    ),
    security(
        ("token" = [])
    )
)]
pub(crate) async fn create_event_handler(
    parts: Parts,
    State(state): State<AppState>,
    Geo(geo): Geo,
    Protobuf(event): Protobuf<AnalyticsEvent>,
) -> Result<impl IntoResponse, AppError> {
    info!("received event: {:?}", event);
    let mut row = AnalyticsEventRow::try_from(event)?;
    row.update_with_server_info(&parts, geo);
    row.set_session_id(&state);

    let mut insert = state.client.insert("analytics_events")?;
    insert.write(&row).await?;
    insert.end().await?;

    Ok(StatusCode::CREATED)
}

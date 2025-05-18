use crate::model::{CreateAgent, UpdateAgent};
use crate::{AppError, AppState, ErrorOutput};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chat_core::ChatAgent;

/// List all chats in the workspace of the user.
#[utoipa::path(
    get,
    path = "/api/chats/{id}/agents",
    responses(
         (status = 200, description = "List of agents", body = Vec<ChatAgent>),
    ),
    tag="agent",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn list_agent_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let agents = state.list_agent(id).await?;
    Ok((StatusCode::OK, Json(agents)))
}

/// Create a new chat in the workspace of the user.
#[utoipa::path(
    post,
    path = "/api/chats/{id}/agents",
    responses(
         (status = 201, description = "Agent created", body = ChatAgent),
    ),
    tag="agent",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn create_agent_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<CreateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.create_agent(input, id).await?;
    Ok((StatusCode::CREATED, Json(agent)))
}

#[utoipa::path(
    patch,
    path = "/api/chats/{id}/agents/{agent_id}",
    params(
         ("id" = u64, Path, description = "Chat id"),
         ("agent_id" = u64, Path, description = "Agent id"),
    ),
    request_body(content = UpdateAgent, description = "update agent", content_type = "application/json"),
    responses(
         (status = 200, description = "Agent found", body = ChatAgent),
         (status = 404, description = "Agent not found", body = ErrorOutput),
    ),
    tag="agent",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn update_agent_handler(
    State(state): State<AppState>,
    Path(_id): Path<u64>,
    Json(input): Json<UpdateAgent>,
) -> Result<impl IntoResponse, AppError> {
    let agent = state.update_agent(input).await?;
    Ok(Json(agent))
}

#[utoipa::path(
    delete,
    path = "/api/chats/{id}/agents/{agent_id}",
    params(
         ("id" = u64, Path, description = "Chat id"),
         ("agent_id" = u64, Path, description = "Agent id"),
    ),
    responses(
         (status = 200, description = "Agent deleted"),
    ),
    tag="agent",
    security(
         ("token" = [])
    )
)]
pub(crate) async fn delete_agent_handler(
    State(state): State<AppState>,
    Path((_id, agent_id)): Path<(u64, u64)>,
) -> Result<impl IntoResponse, AppError> {
    state.delete_agent_by_id(agent_id).await?;
    Ok(StatusCode::OK)
}

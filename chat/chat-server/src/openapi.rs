use crate::handler::*;
use crate::{
    AppState, AuthOutput, CreateChat, CreateMessage, CreateUser, ErrorOutput, ListMessages,
    SigninUser,
};
use axum::Router;
use chat_core::{Chat, ChatType, ChatUser, Message, User, Workspace};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub(crate) trait OpenApiRouter {
    fn openapi(self) -> Self;
}

#[derive(OpenApi)]
#[openapi(
    paths(
        signup_handler,
        signin_handler,
        get_chat_handler,
        update_chat_handler,
        delete_chat_handler,
        create_chat_handler,
        list_message_handler,
        send_message_handler,
        list_chat_handler,
        list_chat_users_handler,
        upload_handler,
        file_handler,
    ),
    components(
        schemas(User, Chat, ChatType, ChatUser, Message, Workspace, SigninUser, CreateUser, CreateChat, CreateMessage, ListMessages, AuthOutput, ErrorOutput),
    ),
    modifiers(&SecurityAddon),
    tags(
             (name = "chat", description = "Chat related operations"),
             (name = "message", description = "Message related operations"),
             (name = "workspace", description = "Workspace related operations"),
             (name = "user", description = "User related operations"),
    )
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

impl OpenApiRouter for Router<AppState> {
    fn openapi(self) -> Self {
        self.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    }
}

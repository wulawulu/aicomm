use crate::middleware::REQUEST_ID_HEADER;
use axum::http::HeaderValue;
use axum::{extract::Request, middleware::Next, response::Response};
use tracing::warn;
use uuid::Uuid;

pub async fn set_request_id(mut req: Request, next: Next) -> Response {
    let option = req.headers().get(REQUEST_ID_HEADER);
    let request_id = match option {
        Some(v) => Some(v.to_owned()),
        None => match HeaderValue::from_str(&Uuid::now_v7().to_string()) {
            Ok(v) => {
                req.headers_mut().insert(REQUEST_ID_HEADER, v.clone());
                Some(v)
            }
            Err(e) => {
                warn!("parse generated request id failed: {}", e);
                None
            }
        },
    };

    let mut response = next.run(req).await;
    let Some(id) = request_id else {
        return response;
    };
    response.headers_mut().insert(REQUEST_ID_HEADER, id);
    response
}

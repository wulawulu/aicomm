//layer

use crate::middleware::{REQUEST_ID_HEADER, SERVER_TIME_HEADER};
use axum::extract::Request;
use axum::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::{Layer, Service};
use tracing::warn;

#[derive(Clone)]
pub(crate) struct ServerTimeLayer;

impl<S> Layer<S> for ServerTimeLayer {
    type Service = ServerTimeMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ServerTimeMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct ServerTimeMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for ServerTimeMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let instant = Instant::now();
        let future = self.inner.call(req);
        Box::pin(async move {
            let mut response: Response = future.await?;
            let elapsed = format!("{}us", instant.elapsed().as_micros());
            println!("Server time: {}", elapsed);
            match elapsed.parse() {
                Ok(v) => {
                    response.headers_mut().insert(SERVER_TIME_HEADER, v);
                }
                Err(e) => {
                    warn!(
                        "Parse elapsed time failed: {} for request {:?}",
                        e,
                        response.headers().get(REQUEST_ID_HEADER)
                    );
                }
            }
            Ok(response)
        })
    }
}

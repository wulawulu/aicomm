use crate::notify::AppEvent;
use crate::{AppState, SenderReceiverCnt};
use axum::extract::State;
use axum::response::{sse::Event, Sse};
use axum::Extension;
use axum_extra::{headers, TypedHeader};
use chat_core::User;
use futures::Stream;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tracing::info;

const CHANNEL_CAPACITY: usize = 256;

struct ReceiverGuard {
    state: AppState,
    user_id: u64,
}

impl Drop for ReceiverGuard {
    fn drop(&mut self) {
        let state = self.state.clone();
        let user_id = self.user_id;
        tokio::spawn(async move {
            let users = &state.users;
            let cnt = {
                let mut entry = users.get_mut(&user_id).unwrap();
                entry.reduce();
                info!("User {} is leaving,current {} session", user_id, entry.cnt);
                entry.cnt
            };
            if cnt == 0 {
                users.remove(&user_id);
                info!("User {} unsubscribed", &user_id);
            }
        });
    }
}

pub(crate) async fn sse_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("`{}` connected", user_agent.as_str());

    let users = &state.users;
    let user_id = user.id as u64;

    let mut entry = users.entry(user_id).or_insert_with(|| {
        let (tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        SenderReceiverCnt::new(tx)
    });
    let tx = entry.clone();
    let rx = tx.subscribe();
    entry.increase();
    info!("User {} subscribed {} times", user_id, entry.cnt);

    let guard = ReceiverGuard {
        state: state.clone(),
        user_id,
    };

    info!("User {} subscribed", user_id);

    let stream = BroadcastStream::new(rx)
        .filter_map(|v| v.ok())
        .map(|v| {
            let name = match v.as_ref() {
                AppEvent::NewChat(_) => "NewChat",
                AppEvent::AddToChat(_) => "AddToChat",
                AppEvent::RemoveFromChat(_) => "RemoveFromChat",
                AppEvent::NewMessage(_) => "NewMessage",
                AppEvent::ChatNameUpdate(_) => "ChatNameUpdate",
            };
            let data = serde_json::to_string(&v).expect("failed to serialize event");
            Ok(Event::default().data(data).event(name))
        })
        .map(move |event| {
            let _ = &guard;
            event
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

use std::{sync::Arc, time::SystemTime};
use axum::{
    extract::{Query, State}, response::IntoResponse, routing::{get, post}, Router
};
use axum_extra::response::ErasedJson;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().pretty().with_max_level(tracing::Level::DEBUG).init();

    // NOTE: binding to 0.0.0.0 binds to *all* networks, meaning you can contact the server at
    // 127.0.0.1 or any IP address this server has. This makes it easy to run a dev server, but has
    // security implications. Had we time we'd at a configuration system for this and default to
    // 127.0.0.1.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        // GET /messages gets all messages.
        .route("/messages", get(list_messages))
        // POST /messages sends a new message.
        .route("/messages", post(send_message))
        .fallback_service(ServeDir::new("client/build/"))
        .with_state(Arc::new(RwLock::new(MessageBoard::default())))
        .layer(TraceLayer::new_for_http())
}

/// Global app data for a message board.
#[derive(Default)]
struct MessageBoard {
    /// All messages in the chatroom.
    messages: Vec<Message>,
}

/// Single message on a message board.
#[derive(Serialize, Deserialize)]
struct Message {
    /// The time the message was sent.
    timestamp: SystemTime,
    /// The message.
    message: String,
}

/// List all messages (optionally starting from a known number).
async fn list_messages(
    Query(params): Query<ListMessagesParams>,
    State(state): State<Arc<RwLock<MessageBoard>>>
) -> impl IntoResponse {
    let messages = &state.read().await.messages;

    // Cap the message ID to prevent out of bounds access panics. Asking for messages above 100 when
    // there are only 2 messages should yield an empty list.
    let first_message_id = params.first_message_id.min(messages.len());

    // NOTE: We use ErasedJson() to proactively serialize the response instead of the lazy Json()
    // usually returned from axum handlers. This is because the messages are behind a lock and are
    // no longer accessible once list_messages() returns (which is when Json() does the serialization).
    //
    // Making this actually lazy would still be good, but probably involves a custom Serialize or
    // IntoResponse implementation.
    ErasedJson::new(&messages[first_message_id..])
}

#[derive(Deserialize)]
struct ListMessagesParams {
    /// First message ID to get. Starts from this point. Defaults to 0.
    #[serde(default)]
    first_message_id: usize,
}

/// Post a message to the board.
async fn send_message(
    State(state): State<Arc<RwLock<MessageBoard>>>,
    message: String
) -> impl IntoResponse {
    // Take a write lock so we can add the message.
    state.write().await.messages.push(Message { timestamp: SystemTime::now(), message });
}


/// Tests
#[cfg(test)]
mod tests;

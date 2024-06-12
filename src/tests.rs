use std::ops::Range;
use axum::{http::StatusCode, Router};
use crate::Message;

#[tokio::test]
async fn empty_message_board() {
    let mut app = crate::app();
    let messages = get_messages(&mut app, "/messages").await;
    assert_eq!(messages.len(), 0);
}

#[tokio::test]
async fn send_and_receive_message() {
    let mut app = crate::app();

    // Send a message
    send_message(&mut app, "/messages", "hi").await;

    // Verify we get the message back
    let messages = get_messages(&mut app, "/messages").await;
    assert_eq!(messages.len(), 1);
}


#[tokio::test]
async fn list_messages() {
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get all messages back
    let messages = get_messages(&mut app, "/messages").await;
    expect_numbered_messages(&messages, 0..10);
}

#[tokio::test]
async fn list_messages_starting_with_0() {
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get all messages back when we ask starting with 0
    let messages = get_messages(&mut app, "/messages?first_message_id=0").await;
    expect_numbered_messages(&messages, 0..10);
}

#[tokio::test]
async fn list_some_messages() {
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get 4 messages back when we ask starting with 6
    let messages = get_messages(&mut app, "/messages?first_message_id=6").await;
    expect_numbered_messages(&messages, 6..10);
}

#[tokio::test]
async fn list_last_messages() {
    // Send 10 messages
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get 1 message back when we ask starting with 9
    let messages = get_messages(&mut app, "/messages?first_message_id=9").await;
    expect_numbered_messages(&messages, 9..10);
}

#[tokio::test]
async fn invalid_first_message_id() {
    // Send 10 messages
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get 1 message back when we ask starting with 9
    let response = http_helpers::get(&mut app, "/messages?first_message_id=x").await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_no_new_messages() {
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get 0 messages back when we ask starting with 10
    let messages = get_messages(&mut app, "/messages?first_message_id=10").await;
    assert_eq!(messages.len(), 0);
}

#[tokio::test]
async fn list_out_of_range_message_id() {
    let mut app = app_with_numbered_messages(0..10).await;

    // Verify we get 0 messages back when we ask starting with 11
    let messages = get_messages(&mut app, "/messages?first_message_id=11").await;
    assert_eq!(messages.len(), 0);
}

//
// Helpers
//

async fn app_with_numbered_messages(range: Range<usize>) -> Router {
    let mut app = crate::app();
    for i in range {
        send_message(&mut app, "/messages", &format!("message {}", i)).await;
    }
    app
}

fn expect_numbered_messages(messages: &[Message], expected_range: Range<usize>) {
    assert_eq!(messages.len(), expected_range.len());
    for (message, expected_number) in messages.iter().zip(expected_range.into_iter()) {
        assert_eq!(message.message, format!("message {}", expected_number));
    }
}

// Get messages from the server, verifying that the response is OK and deserializing the JSON.
async fn get_messages(app: &mut Router, uri: &str) -> Vec<Message> {
    let response = http_helpers::get_ok(app, uri).await;
    http_helpers::json_response(response).await
}

async fn send_message(app: &mut Router, uri: &str, message: &str) {
    http_helpers::send_message(app, uri, message).await;
}

// Generic http helpers
mod http_helpers {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode}, response::{IntoResponse, Response}, Router,
    };
    use http_body_util::BodyExt; use serde::de::DeserializeOwned;
    // for `collect`
    use tower::{Service, ServiceExt};
    /// Make a request to the app and return the response.
    pub async fn call(app: &mut Router, request: Request<Body>) -> Response<Body> {
        ServiceExt::<Request<Body>>::ready(app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap()
    }

    // Make a get request to the given URI and return the response.
    pub async fn get(app: &mut Router, uri: &str) -> Response<Body> {
        call(app, get_request(uri)).await
    }

    // Make a get request to the givern URI, verify the status is OK, and return the response.
    pub async fn get_ok(app: &mut Router, uri: &str) -> Response<Body> {
        let response = get(app, uri).await;
        assert_eq!(response.status(), StatusCode::OK);
        response
    }

    pub async fn post(app: &mut Router, uri: &str, body: &str) -> Response<Body> {
        call(app, post_request(uri, body)).await
    }
    pub async fn send_message(app: &mut Router, uri: &str, body: &str) -> Response<Body> {
        let response = post(app, uri, body).await;
        assert_eq!(response.status(), StatusCode::OK);
        response
    }

    // async fn post_json<T: Serialize>(app: Router, uri: &str, body: T) -> Response<Body> {
    //     call_request(app, post_json_request(uri, body)).await
    // }

    pub fn get_request(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    pub fn post_request(uri: &str, body: impl Into<String>) -> Request<Body> {
        Request::builder()
            .method(http::Method::POST)
            .uri(uri)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(body.into()))
            .unwrap()
    }

    // fn post_json_request<T: Serialize>(uri: &str, body: T) -> Request<Body> {
    //     post_request(uri, serde_json::to_string(&body).unwrap())
    // }

    pub async fn json_response<T: DeserializeOwned>(response: impl IntoResponse) -> T {
        let response = response.into_response();
        let x = response.into_body().collect().await;
        let bytes = x.unwrap().to_bytes();
        serde_json::from_slice(bytes.as_ref()).unwrap()
    }
}

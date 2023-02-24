use async_trait::async_trait;
use futures::{stream::SplitStream, StreamExt};
use log::{debug, error};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::{error_action::WebSocketError, receive_trait::WebOsSocketTvReceive};

fn message_to_json(message: Message) -> Result<Value, WebSocketError> {
    match message {
        Message::Text(text_message) => match serde_json::from_str::<Value>(&text_message) {
            Ok(json) => Ok(json),
            Err(e) => {
                error!("Fail to convert to json, message: {text_message} ::: Error: {e:?}");
                Err(WebSocketError::Continue)
            }
        },
        Message::Ping(bytes) => {
            debug!("Recived ping: {bytes:?}");
            Err(WebSocketError::Continue)
        }

        Message::Close(frame) => {
            debug!("socket desconecting... frame: {frame:?}");
            Err(WebSocketError::Fatal)
        }

        Message::Pong(bytes) => {
            debug!("Recived pong: {bytes:?}");
            Err(WebSocketError::Continue)
        }
        Message::Binary(bytes) => {
            debug!("binary message: {bytes:?}");
            Err(WebSocketError::Continue)
        }
    }
}

#[async_trait]
impl WebOsSocketTvReceive for SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    async fn receive(&mut self) -> Result<Value, WebSocketError> {
        // let mut stream = stream;

        match self.next().await {
            Some(result_message) => match result_message {
                Ok(message) => message_to_json(message),
                Err(e) => Err(WebSocketError::from(e)),
            },
            None => Err(WebSocketError::Continue),
        }
    }
}

#[test]
fn test_message_to_json() {
    let expected_continue = |result: Result<Value, WebSocketError>| match result {
        Ok(_) => false,
        Err(e) => match e {
            WebSocketError::Fatal => false,
            WebSocketError::Continue => true,
        },
    };

    let expected_faltal = |result| match result {
        Ok(_) => false,
        Err(e) => match e {
            WebSocketError::Fatal => true,
            WebSocketError::Continue => false,
        },
    };

    let expected_json = |result| match result {
        Ok(_) => true,
        Err(e) => match e {
            WebSocketError::Fatal => false,
            WebSocketError::Continue => false,
        },
    };

    let results = vec![
        message_to_json(Message::Ping(vec![])),
        message_to_json(Message::Pong(vec![])),
        message_to_json(Message::Binary(vec![1, 2, 3])),
        message_to_json(Message::Close(None)),
        message_to_json(Message::Text("It's not a json".to_string())),
        message_to_json(Message::Text("{\"text\":\"it's a json\"}".to_string())),
    ];

    let expected_result = [
        expected_continue, // ping
        expected_continue, // pong
        expected_continue, // Binary
        expected_faltal,   // Close
        expected_continue, // Text not json
        expected_json,     // Text json
    ];

    for (result, expected_fn) in results.iter().zip(expected_result) {
        assert_eq!(true, expected_fn(result.to_owned()));
    }
}

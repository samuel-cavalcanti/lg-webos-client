use futures::{Stream, StreamExt};

use futures_util::future::ready;
use log::debug;
use pinky_swear::Pinky;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;

use crate::lg_command::{CommandRequest, CommandResponse};

pub async fn process_messages_from_server<T>(
    stream: T,
    pending_requests: Arc<Mutex<HashMap<u8, Pinky<CommandResponse>>>>,
    registration_pinky: Pinky<Option<String>>,
) where
    T: Stream<Item = Result<Message, Error>>,
{
    stream
        .for_each(|message| match message {
            Ok(_message) => {
                if let Ok(text_message) = _message.into_text() {
                    if let Ok(json) = serde_json::from_str::<Value>(&text_message) {
                        debug!("JSON Response: {}", json);

                        if json["type"] == "registered" {
                            let key = json
                                .get("payload")
                                .and_then(|p| p.get("client-key"))
                                .and_then(|k| k.as_str())
                                .map(Into::into);
                            registration_pinky.swear(key);
                        } else {
                            let mut error: bool = false;
                            let res = match json["id"].as_i64() {
                                Some(r) => r,
                                None => {
                                    error = true;
                                    0
                                }
                            };
                            if !error {
                                let response = CommandResponse {
                                    id: res as u8,
                                    payload: Some(json["payload"].clone()),
                                };
                                let requests = pending_requests.lock().unwrap();
                                requests.get(&response.id).unwrap().swear(response);
                            }
                        }
                    }
                }
                ready(())
            }
            Err(_) => ready(()),
        })
        .await
}

/// Get the initial handshake packet for connecting to a device.
/// A client-key can be set by something similar to
/// `get_handshake()["payload"]["client-key"] = ...`
/// # Return
/// The initial handshake packet needed to connect to a WebOS device.
pub fn get_handshake() -> serde_json::Value {
    serde_json::from_str(include_str!("../handshake.json")).expect("Could not parse handshake json")
}
impl From<&CommandRequest> for Message {
    fn from(request: &CommandRequest) -> Self {
        Message::text(serde_json::to_string(request).unwrap())
    }
}

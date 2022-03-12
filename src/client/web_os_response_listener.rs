use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use pinky_swear::Pinky;
use serde_json::Value;

use tokio_tungstenite::tungstenite::{Error, Message};
use futures::{Stream, StreamExt};
use log::debug;
use futures_util::future::ready;

pub async fn process_messages_from_server<T>(
    stream: T,
    pending_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
) where
    T: Stream<Item=Result<Message, Error>>,
{
    let mut authentication_message_is_received = false;
    stream
        .for_each(|message| match message {
            Ok(_message) => {
                if let Ok(text_message) = _message.into_text() {
                    match serde_json::from_str::<Value>(&text_message) {
                        Ok(json) => {
                            debug!("JSON Response: {}", json);
                            /*
                                when handshake occurs, the TV can send 2 requests.
                                Because I use an HashMap<u8, Pinky<Value>, the first response
                                resolve the promise, but is the second response what do have the
                                authentication data, so we wait the authentication response arrives.
                             */
                            authentication_message_is_received = check_authentication_is_received(authentication_message_is_received, json["type"].as_str());
                            if authentication_message_is_received {
                                accept_promises(json, pending_requests.clone());
                            }
                        }
                        Err(..) => {
                            debug!("Unable do Convert message to Json {}", text_message);
                        }
                    }
                }
                ready(())
            }
            Err(_) => ready(()),
        })
        .await
}


fn accept_promises(json: Value,
                   pending_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>)
{
    match json["id"].as_u64() {
        Some(res) => {
            let requests = pending_requests.lock().unwrap();
            let res = res as u8;

            match requests.get(&res) {
                None => {
                    debug!("unable to find request of id {}", res);
                }
                Some(promise) => {
                    promise.swear(json);
                    debug!("resolved promise id: {}",res);
                }
            };
        }//.swear(json)
        None => {
            debug!("JSON response not have id");
        }
    };
}


fn check_authentication_is_received(authentication_message_is_received: bool, response_type: Option<&str>) -> bool {
    debug!("Current authentication is received:  {}",authentication_message_is_received);
    return match authentication_message_is_received {
        true => { true }
        false => {
            if let Some(response_type) = response_type {
                if response_type == "registered" {
                    return true;
                } else if response_type == "error" {
                    return true;
                }
            }
            return false;
        }
    };
}
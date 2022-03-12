use std::cell::RefCell;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};


use futures_util::{SinkExt, StreamExt};

use futures_util::stream::{SplitSink};
use log::{debug};
use pinky_swear::{Pinky, PinkySwear};
use serde_json::{json, Value};
use tokio;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use tokio_tungstenite::tungstenite::Message;


use crate::client::web_os_response_listener;

pub struct WebOsTVRequestSender {
    write: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    next_command_id: Arc<Mutex<u8>>,
    ongoing_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
    pub key: Option<String>,
}


pub async fn connect_to_tv(address: &String) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let url = url::Url::parse(address).expect("Could not parse given address");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");


    ws_stream
}

pub fn init_sender_and_listener(ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> WebOsTVRequestSender {
    let next_command_id = Arc::from(Mutex::from(0));
    let ongoing_requests = Arc::from(Mutex::from(HashMap::new()));
    let requests_to_process = ongoing_requests.clone();
    let (write, read) = ws_stream.split();

    let sender = WebOsTVRequestSender::new(RefCell::new(write),
                                           next_command_id,
                                           ongoing_requests);


    tokio::spawn(async move {
        /*
            in this new thread,  the server is going to wait new messages from TV and
            complete the promises when they arrived.
         */
        debug!("listener has start");
        web_os_response_listener::process_messages_from_server(read, requests_to_process).await
    });

    sender
}

impl WebOsTVRequestSender {
    pub fn new(write: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
               command_id: Arc<Mutex<u8>>,
               ongoing_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
    ) -> WebOsTVRequestSender
    {
        WebOsTVRequestSender {
            write,
            next_command_id: command_id,
            ongoing_requests,
            key: None,
        }
    }

    pub async fn send_json(&self, json: Value) -> PinkySwear<Value> {
        let next_command_id = self.generate_next_id();
        let (promise, pinky) = PinkySwear::<Value>::new();

        self.ongoing_requests
            .lock()
            .unwrap()
            .insert(next_command_id, pinky);

        let mut json = json;
        json["id"] = Value::from(next_command_id);
        debug!("sending json {}",json);
        let message = Message::text(serde_json::to_string(&json).unwrap());

        self.write.borrow_mut().send(message).await.expect("Unable to send message");


        promise
    }

    fn generate_next_id(&self) -> u8 {
        let mut guard = self
            .next_command_id
            .lock()
            .expect("Could not lock next_command_id");
        let value = *guard;
        *guard = (1 + value) % 255;

        value
    }
}



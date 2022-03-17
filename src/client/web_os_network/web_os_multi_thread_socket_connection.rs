use futures::StreamExt;
use log::debug;
use tokio::net::TcpStream;
use url::Url;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::client::web_os_network::web_os_request_response_cycle::{
    WebOsSocketResponseListener, WebOsTVRequestSender,
};
use crate::client::web_os_network::web_os_tv_socket::{WebSocketTvReceive, WebSocketTvSend};
use crate::client::web_os_network::WebOsSocketTvReceive;
use crate::client::{WebOsClientConfig, WebSocketErrorAction};

use super::connection::Connection;
use super::handshake::HandShake;

pub struct WebOsMultiThreadSocketConnection;

impl WebOsMultiThreadSocketConnection {
    pub async fn connect_to_tv(config: WebOsClientConfig) -> Result<Connection, String> {
        let url = WebOsMultiThreadSocketConnection::try_to_parse_url(&config.address)?;
        let ws_stream = WebOsMultiThreadSocketConnection::try_to_connect(url).await?;

        let (write, read) = ws_stream.split();

        let mut sender = WebSocketTvSend::new(write);
        let mut web_socket_receive = WebSocketTvReceive::new(read);

        let key =
            HandShake::do_the_handshake(&mut sender, &mut web_socket_receive, config.key).await?;

        debug!("WebSocket handshake has been successfully completed");

        let ongoing_requests = Arc::new(Mutex::new(HashMap::new()));

        let listener = WebOsSocketResponseListener::new(ongoing_requests.clone());
        let request_sender =
            WebOsTVRequestSender::new(Box::new(sender), Arc::new(Mutex::new(0)), ongoing_requests);

        tokio::spawn(async move {
            WebOsMultiThreadSocketConnection::listener_loop(listener, web_socket_receive).await;
        });

        Ok(Connection {
            sender: Box::new(request_sender),
            key,
        })
    }

    fn try_to_parse_url(address: &String) -> Result<Url, String> {
        match url::Url::parse(address) {
            Ok(url) => Ok(url),
            Err(_) => Err(format!("Could not parse given address {}", address).to_string()),
        }
    }
    async fn try_to_connect(
        url: Url,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
        match connect_async(&url).await {
            Ok((ws_stream, _)) => Ok(ws_stream),
            Err(_) => Err(format!("Unable to Connect to {:?}", url).to_string()),
        }
    }

    async fn listener_loop(
        listener: WebOsSocketResponseListener,
        mut web_socket_receive: WebSocketTvReceive,
    ) {
        /*
           in this new thread,  the server is going to wait new messages from TV and
           complete the promises when they arrived.
        */
        debug!("listener has start");

        loop {
            match web_socket_receive.receive().await {
                Ok(json) => listener.receive_from_tv(json),
                Err(e) => {
                    if let WebSocketErrorAction::Fatal = e {
                        debug!("listener has stopped by Fatal error");
                        return;
                    }
                }
            }
        }
    }
}

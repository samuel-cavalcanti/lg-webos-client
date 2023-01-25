use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

use super::WebSocketErrorAction;

pub struct WebSocketConnection;

impl WebSocketConnection {
    fn try_to_parse_url(address: &String) -> Result<Url, WebSocketErrorAction> {
        match url::Url::parse(address) {
            Ok(url) => Ok(url),
            Err(e) => {
                error!("Could not parse given address {address} error {e}");

                Err(WebSocketErrorAction::Continue)
            }
        }
    }

    pub async fn try_to_connect(
        address: &String,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, WebSocketErrorAction> {
        let url = WebSocketConnection::try_to_parse_url(address)?;

        match connect_async(&url).await {
            Ok((ws_stream, _)) => Ok(ws_stream),
            Err(_) => Err(WebSocketErrorAction::Fatal),
        }
    }
}

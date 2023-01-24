use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub struct WebSocketConnection;

impl WebSocketConnection {
    fn try_to_parse_url(address: &String) -> Result<Url, String> {
        match url::Url::parse(address) {
            Ok(url) => Ok(url),
            Err(_) => Err(format!("Could not parse given address {address}")),
        }
    }

    pub async fn try_to_connect(
        address: &String,
    ) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, String> {
        let url = WebSocketConnection::try_to_parse_url(address)?;

        match connect_async(&url).await {
            Ok((ws_stream, _)) => Ok(ws_stream),
            Err(_) => Err(format!("Unable to Connect to {url:?}")),
        }
    }
}

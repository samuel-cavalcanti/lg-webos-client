use log::error;
use tokio_tungstenite::tungstenite;

#[derive(Debug, Clone)]
/// Error on WebSocket Connection
/// There are two kinds of erros: Fatal and Continue
pub enum WebSocketError {
    /// When A Fatal error occurs, then you must close the WebSocket Connection  
    /// Maybe Restart the Connection works
    Fatal,
    /// When A Continue Error Occurs, the you can ignore it
    Continue,
}

impl WebSocketError {
    pub fn from(error: tungstenite::Error) -> WebSocketError {
        match error {
            tungstenite::Error::ConnectionClosed => WebSocketError::Fatal,
            tungstenite::Error::AlreadyClosed => WebSocketError::Fatal,
            tungstenite::Error::Io(_) => WebSocketError::Fatal,
            tungstenite::Error::Tls(e) => {
                error!("Tls Error: {}", e);
                WebSocketError::Continue
            }
            tungstenite::Error::Capacity(e) => {
                error!("Capacity Error: {}", e);
                WebSocketError::Continue
            }
            tungstenite::Error::Protocol(e) => {
                error!("Web socket protocol Error: {}", e);
                WebSocketError::Continue
            }
            tungstenite::Error::SendQueueFull(m) => {
                error!("Send Queue Is Full {}", m);
                WebSocketError::Continue
            }
            tungstenite::Error::Utf8 => {
                error!("Utf 8 coding error ");
                WebSocketError::Continue
            }
            tungstenite::Error::Url(u) => {
                error!("Invalid Url error {}", u);
                WebSocketError::Continue
            }
            tungstenite::Error::Http(http_response) => {
                error!("Http error {:?}", http_response);
                WebSocketError::Continue
            }
            tungstenite::Error::HttpFormat(e) => {
                error!("Http Format Error {}", e);
                WebSocketError::Continue
            }
        }
    }
}

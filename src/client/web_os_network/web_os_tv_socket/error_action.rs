use tokio_tungstenite::tungstenite;

use log::error;

#[derive(Debug, Clone)]
pub enum WebSocketErrorAction {
    /*
        Some Networks erros can be ignored, so I use ResponseErrosAction::Continue,
        otherwise  the networking connection must be finish.

    */
    Fatal,
    Continue,
}

impl WebSocketErrorAction {
    pub fn from(error: tungstenite::Error) -> WebSocketErrorAction {
        match error {
            tungstenite::Error::ConnectionClosed => WebSocketErrorAction::Fatal,
            tungstenite::Error::AlreadyClosed => WebSocketErrorAction::Fatal,
            tungstenite::Error::Io(_) => WebSocketErrorAction::Fatal,
            tungstenite::Error::Tls(e) => {
                error!("Tls Error: {}", e);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::Capacity(e) => {
                error!("Capacity Error: {}", e);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::Protocol(e) => {
                error!("Web socket protocol Error: {}", e);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::SendQueueFull(m) => {
                error!("Send Queue Is Full {}", m);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::Utf8 => {
                error!("Utf 8 coding error ");
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::Url(u) => {
                error!("Invalid Url error {}", u);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::Http(http_response) => {
                error!("Http error {:?}", http_response);
                WebSocketErrorAction::Continue
            }
            tungstenite::Error::HttpFormat(e) => {
                error!("Http Format Error {}", e);
                WebSocketErrorAction::Continue
            }
        }
    }
}

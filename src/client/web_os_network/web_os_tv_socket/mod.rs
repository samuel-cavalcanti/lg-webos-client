/*
    Tv communication occurs sending and Receiving Json Objects  between an WebSocket Communication
*/
mod error_action;
pub mod websocket_receive;
pub mod websocket_send;

mod receive_trait;
mod send_trait;
mod tv_send_to_websocket_stream;

pub use error_action::WebSocketError;
pub use receive_trait::WebOsSocketTvReceive;
pub use send_trait::WebOsSocketTvSend;

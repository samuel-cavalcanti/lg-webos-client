/*
    Tv communication occurs sending and Receiving Json Objects  between an WebSocket Communication
*/
mod error_action;
pub mod websocket_receive;
pub mod websocket_send;

mod tv_send_to_websocket_stream;
mod receive_trait;
mod send_trait;

pub use error_action::WebSocketErrorAction;
pub use receive_trait::WebOsSocketTvReceive;
pub use send_trait::WebOsSocketTvSend;

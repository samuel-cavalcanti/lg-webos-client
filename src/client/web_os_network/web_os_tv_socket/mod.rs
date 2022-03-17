
/*
    Tv communication occurs sending and Receiving Json Objects  between an WebSocket Communication 
*/
mod error_action;
mod websocket_receive;
mod websocket_send;

mod receive_trait;
mod send_trait;


pub use receive_trait::WebOsSocketTvReceive;
pub use send_trait::WebOsSocketTvSend;
pub use error_action::WebSocketErrorAction;
pub use websocket_receive::WebSocketTvReceive;
pub use websocket_send::WebSocketTvSend;
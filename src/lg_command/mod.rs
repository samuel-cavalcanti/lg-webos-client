//! # LG Command Module
//!
//! This module has all commands that can be sended to the WebOs TV.
//! The [`CommandRequest`] is the expected json that's what the TV waiting for and
//! the [`CommandResponse`] is the Json that the client expected.
//! The Command response and Command request is sended though the main WebSocket connection.
//! There Are a lot of commands that the TV can receive, so I created the abstraction:
//!  - the [`LGCommandRequest`] trait
//! This trait just return a [`CommandRequest`]. See all commands is see all the structs that
//! implements the trait. These structs is in `request_commands` module.
//!
mod command_request;
mod command_response;
mod lg_command_request_trait;
pub mod request_commands;

pub mod pointer_input_commands;

pub use command_request::CommandRequest;
pub use command_response::CommandResponse;
pub use lg_command_request_trait::LGCommandRequest;

pub const REQUEST_TYPE: &str = "request";

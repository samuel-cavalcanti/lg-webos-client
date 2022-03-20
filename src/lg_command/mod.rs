mod command_request;
mod command_response;
mod lg_command_request_trait;
pub mod request_commands;

/*
   Pointer Input Socket seems to be the TV remote Controller
   like, Home,Back,Enter button.
*/
pub mod pointer_input_commands;

pub use command_request::CommandRequest;
pub use command_response::CommandResponse;
pub use lg_command_request_trait::LGCommandRequest;

pub const REQUEST_TYPE: &str = "request";

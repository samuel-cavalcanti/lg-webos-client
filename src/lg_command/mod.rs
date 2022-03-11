mod lg_command_request_trait;
mod command_request;
mod command_response;
pub mod commands;

pub use lg_command_request_trait::LGCommandRequest;
pub use command_request::CommandRequest;
pub use command_response::CommandResponse;


pub const REQUEST_TYPE: &str = "request";

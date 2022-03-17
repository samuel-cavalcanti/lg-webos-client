mod command_request;
mod command_response;
pub mod commands;
mod lg_command_request_trait;

pub use command_request::CommandRequest;
pub use command_response::CommandResponse;
pub use lg_command_request_trait::LGCommandRequest;

pub const REQUEST_TYPE: &str = "request";

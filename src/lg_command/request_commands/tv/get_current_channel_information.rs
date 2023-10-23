use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

/// Use this command to receive a Json with the current channel information
pub struct GetCurrentChannelInformation;

impl LGCommandRequest for GetCurrentChannelInformation {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getCurrentChannel"),
            payload: None,
        }
    }
}

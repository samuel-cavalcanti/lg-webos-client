use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

/// Use this command to receive a Json with all channels avalable.
pub struct GetChannelList;

impl LGCommandRequest for GetChannelList {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getChannelList"),
            payload: None,
        }
    }
}

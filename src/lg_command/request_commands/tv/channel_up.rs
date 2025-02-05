use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct ChannelUp;

impl LGCommandRequest for ChannelUp {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/channelUp"),
            payload: None,
        }
    }
}

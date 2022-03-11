use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct ChannelDown;


impl LGCommandRequest for ChannelDown {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/channelDown"),
            payload: None,
        }
    }
}
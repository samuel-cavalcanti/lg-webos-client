use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct SetOpenChannel {
    pub channel_id: String,
}

impl LGCommandRequest for SetOpenChannel {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/openChannel"),
            payload: Some(json!({ "channelId": self.channel_id })),
        }
    }
}

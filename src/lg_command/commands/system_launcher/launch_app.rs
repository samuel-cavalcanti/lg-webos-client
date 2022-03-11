use serde_json::{json, Value};
use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

pub struct LaunchApp {
    pub app_id: String,
    pub parameters: Value,
}


impl LGCommandRequest for LaunchApp {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.launcher/launch"),
            payload: Some(json!({ "id": self.app_id, "params": self.parameters })),
        }
    }
}
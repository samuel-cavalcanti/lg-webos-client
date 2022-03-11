use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct CreateToast {
    pub message: String,
}

impl LGCommandRequest for CreateToast {
    fn to_command_request(&self, id: u8) -> CommandRequest {
        CommandRequest {
            id,
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.notifications/createToast"),
            payload: Some(json!({ "message": self.message })),
        }
    }
}

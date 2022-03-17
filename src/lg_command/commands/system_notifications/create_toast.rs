use serde_json::json;

use crate::lg_command::REQUEST_TYPE;
use crate::lg_command::{CommandRequest, LGCommandRequest};

pub struct CreateToast {
    pub message: String,
}

impl LGCommandRequest for CreateToast {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.notifications/createToast"),
            payload: Some(json!({ "message": self.message })),
        }
    }
}

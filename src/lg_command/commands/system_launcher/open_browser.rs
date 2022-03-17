use serde_json::json;

use crate::{lg_command::{CommandRequest, LGCommandRequest}};
use crate::lg_command::REQUEST_TYPE;

pub struct OpenBrowser {
    pub url: String,
}

impl LGCommandRequest for OpenBrowser {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
         
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.launcher/open"),
            payload: Some(json!({ "target": self.url })),
        }
    }
}

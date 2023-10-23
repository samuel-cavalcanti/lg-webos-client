use crate::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};
use serde_json::{json, Value};

/// Open an Aplication on WebOs.
/// To Launch an App you must to know the **app_id**
/// You can get know all the avalable apps using `ListApps` commands
pub struct LaunchApp {
    pub app_id: Option<String>,
    pub name: Option<String>,
    pub parameters: Value,
}

impl LGCommandRequest for LaunchApp {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.launcher/launch"),
            payload: Some(
                json!({ "id": self.app_id, "name":self.name, "params": self.parameters }),
            ),
        }
    }
}

impl LaunchApp {
    pub fn youtube() -> LaunchApp {
        LaunchApp {
            app_id: Some("youtube.leanback.v4".to_string()),
            name: None,
            parameters: json!({}),
        }
    }

    pub fn netflix() -> LaunchApp {
        LaunchApp {
            app_id: Some("netflix".to_string()),
            name: None,
            parameters: json!({}),
        }
    }

    pub fn amazon_prime_video() -> LaunchApp {
        LaunchApp {
            app_id: Some("amazon".to_string()),
            name: None,
            parameters: json!({}),
        }
    }
}

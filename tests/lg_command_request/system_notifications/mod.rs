use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::commands;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};
use serde_json::json;

#[test]
fn test_create_toast_command() {
    for message in ["Hello", "My", "TV"] {
        let message = message.to_string();

        let toast = commands::system_notifications::CreateToast {
            message: message.clone(),
        };

        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: "ssap://system.notifications/createToast".to_string(),
            payload: Some(json!({ "message": message })),
        };

        let result = toast.to_command_request();

        assert_command_request(result, expected);
    }
}

use serde_json::json;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};
use lg_webos_client::lg_command::commands;
use crate::lg_command_request::assert_command_request;

#[test]
fn test_create_toast_command() {
    let toast = commands::system_notifications::CreateToast {
        message: "Olá TV!".to_string(),
    };

    let expected = CommandRequest {
        id: 10,
        r#type: REQUEST_TYPE.to_string(),
        uri: "ssap://system.notifications/createToast".to_string(),
        payload: Some(json!({"message":toast.message})),
    };

    let result = toast.to_command_request(expected.id);

    assert_command_request(result, expected);
}


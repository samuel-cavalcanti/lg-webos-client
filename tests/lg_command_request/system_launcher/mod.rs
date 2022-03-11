use crate::lg_command_request::assert_command_request;
use lg_webos_client::{lg_command::{CommandRequest, LGCommandRequest}};
use serde_json::json;
use lg_webos_client::lg_command::{commands, REQUEST_TYPE};



#[test]
fn test_open_browser() {
    let url = "https://www.google.com".to_string();
    let expected = CommandRequest {
        id: 10,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://system.launcher/open"),
        payload: Some(json!({ "target": url.clone()})),
    };

    let command = commands::system_launcher::OpenBrowser { url };

    let result = command.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_launch_app() {
    let expected = CommandRequest {
        id:1,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://system.launcher/launch"),
        payload: Some(json!({ "id": "netflix", "params": {} })),
    };

    let result = commands::system_launcher::LaunchApp {
        app_id: "netflix".to_string(),
        parameters: json!({}),
    }.to_command_request(expected.id);

    assert_command_request(result, expected);
}
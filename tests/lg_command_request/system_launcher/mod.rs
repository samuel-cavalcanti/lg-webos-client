use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{commands, REQUEST_TYPE};
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};
use serde_json::json;

#[test]
fn test_open_browser() {
    for url in [
        "https://www.google.com",
        "https://www.youtube.com",
        "https://www.gmail.com",
    ] {
        let url = url.to_string();
        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.launcher/open"),
            payload: Some(json!({ "target": url.clone()})),
        };

        let command = commands::system_launcher::OpenBrowser { url };

        let result = command.to_command_request();

        assert_command_request(result, expected);
    }
}

#[test]
fn test_launch_app() {
    for app_id in ["netflix", "amazon", "youtube"] {
        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://system.launcher/launch"),
            payload: Some(json!({ "id": app_id, "params": {} })),
        };

        let result = commands::system_launcher::LaunchApp {
            app_id: app_id.to_string(),
            parameters: json!({}),
        }
        .to_command_request();

        assert_command_request(result, expected);
    }
}

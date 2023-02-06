use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::request_commands::system_launcher;
use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};
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

        let command = request_commands::system_launcher::OpenBrowser { url };

        let result = command.to_command_request();

        assert_command_request(result, expected);
    }
}

#[test]
fn test_launch_app() {
    let apps = [
        system_launcher::LaunchApp::netflix(),
        system_launcher::LaunchApp::amazon_prime_video(),
        system_launcher::LaunchApp::youtube(),
    ];

    let app_id = ["netflix", "amazon", "youtube.leanback.v4"].map(|id| CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://system.launcher/launch"),
        payload: Some(json!({ "id": id, "name": serde_json::Value::Null,  "params": {} })),
    });

    for (app, request) in apps.iter().zip(app_id) {
        let result = app.to_command_request();
        assert_command_request(result, request);
    }
}

#[test]
fn test_list_app() {
    let command = system_launcher::ListApps;

    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.applicationManager/listApps"),
        payload: None,
    };

    assert_command_request(command.to_command_request(), expected);
}

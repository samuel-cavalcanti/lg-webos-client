use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};

use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};
use serde_json::json;

#[test]
fn test_set_display_3d() {
    let expected_requests = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.tv.display/set3DOn"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.tv.display/set3DOff"),
            payload: None,
        },
    ];

    let commands: Vec<request_commands::web_os_services::SetDisplay3D> = vec![true, false]
        .iter()
        .map(|&turn_3d| request_commands::web_os_services::SetDisplay3D { turn_3d })
        .collect();

    for (command, request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), request);
    }
}

#[test]
fn no_payload_commands() {
    let expected_requests = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.update/getCurrentSWInformation"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.networkinput/getPointerInputSocket"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.ime/sendEnterKey"),
            payload: None,
        },
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::web_os_services::GetCurrentServicesInformationList),
        Box::new(request_commands::web_os_services::GetPointerInputSocketUri),
        Box::new(request_commands::web_os_services::SendEnterKey),
    ];

    for (command, request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), request);
    }
}

#[test]
fn insert_text() {
    let expected_requests = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.ime/insertText"),
            payload: Some(json!(
                {
                    "replace":0,
                    "text": String::from("test")
                }
            )),
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.ime/insertText"),
            payload: Some(json!(
                {
                    "replace":1,
                    "text": String::from("test 2")
                }
            )),
        },
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::web_os_services::InsertText {
            text: "test".to_string(),
            replace: false,
        }),
        Box::new(request_commands::web_os_services::InsertText {
            text: "test 2".to_string(),
            replace: true,
        }),
    ];

    for (command, request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), request);
    }
}

#[test]
fn delete_characters() {
    let expected_requests = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.ime/deleteCharacters"),
            payload: Some(json!({"count":5})),
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.ime/deleteCharacters"),
            payload: Some(json!({"count":2})),
        },
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::web_os_services::DeleteCharacters { number_of_chars: 5 }),
        Box::new(request_commands::web_os_services::DeleteCharacters { number_of_chars: 2 }),
    ];


    for (command, request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), request);
    }
}

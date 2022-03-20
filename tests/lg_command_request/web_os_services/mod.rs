use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};

use lg_webos_client::lg_command::{request_commands, REQUEST_TYPE};

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
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::web_os_services::GetCurrentServicesInformationList),
        Box::new(request_commands::web_os_services::GetPointerInputSocketUri),
    ];

    for (command, request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), request);
    }
}

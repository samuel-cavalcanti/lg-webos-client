use crate::lg_command_request::assert_command_request;
use lg_webos_client::{lg_command::{CommandRequest, LGCommandRequest}};

use lg_webos_client::lg_command::{commands, REQUEST_TYPE};


#[test]
fn test_set_display_3d_on() {
    let expected = CommandRequest {
        id: 12,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.tv.display/set3DOn"),
        payload: None,
    };

    let result = commands::web_os_services::SetDisplay3D {
        turn_3d: true
    }.
        to_command_request(expected.id);

    assert_command_request(result, expected);
}


#[test]
fn test_set_display_3d_off() {
    let expected = CommandRequest {
        id: 123,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.tv.display/set3DOff"),
        payload: None,
    };

    let result = commands::web_os_services::SetDisplay3D {
        turn_3d: false
    }.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_services_list() {
    let expected = CommandRequest {
        id: 12,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.update/getCurrentSWInformation"),
        payload: None,
    };

    let result = commands::web_os_services::GetCurrentServicesInformationList.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_pointer_socket_uri() {
    let expected = CommandRequest {
        id: 45,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://com.webos.service.networkinput/getPointerInputSocket"),
        payload: None,
    };

    let result = commands::web_os_services::GetPointerInputSocketUri.to_command_request(expected.id);

    assert_command_request(result, expected);
}
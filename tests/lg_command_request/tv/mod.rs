use lg_webos_client::{lg_command::{CommandRequest, LGCommandRequest}};
use serde_json::json;
use lg_webos_client::lg_command::{ REQUEST_TYPE};
use lg_webos_client::lg_command::commands;
use crate::lg_command_request::assert_command_request;


#[test]
fn test_set_open_channel() {
    let open_channel_id = "1_21_6_1_1519_48608_1519".to_string();
    let expected = CommandRequest {
        id: 13,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/openChannel"),
        payload: Some(json!({ "channelId": open_channel_id.clone() })),
    };

    let result = commands::tv::SetOpenChannel {
        channel_id: open_channel_id,
    }
        .to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_switch_input() {
    let input_id = "test".to_string();

    let expected = CommandRequest {
        id: 50,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/switchInput"),
        payload: Some(json!({ "inputId":input_id.clone() })),
    };

    let result = commands::tv::SwitchInput { input_id }.to_command_request(expected.id);

    assert_command_request(result, expected);
}


#[test]
fn test_get_channel_list() {
    let expected = CommandRequest {
        id: 43,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/getChannelList"),
        payload: None,
    };

    let result = commands::tv::GetChannelList.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_current_channel_information() {
    let expected = CommandRequest {
        id: 33,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/getCurrentChannel"),
        payload: None,
    };

    let result = commands::tv::GetCurrentChannelInformation.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_open_channel_information() {
    let channel_id = "test".to_string();
    let expected = CommandRequest {
        id: 254,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/openChannel"),
        payload: Some(json!({ "channelId": channel_id.clone() })),
    };

    let result = commands::tv::GetOpenChannelInformation { channel_id }.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_channel_up() {
    let expected =  CommandRequest {
        id:1,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/channelUp"),
        payload: None,
    };

    let result = commands::tv::ChannelUp.to_command_request(expected.id);

    assert_command_request(result,expected);
}

#[test]
fn test_channel_down() {
    let expected = CommandRequest {
        id: 2,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/channelDown"),
        payload: None,
    };

    let result = commands::tv::ChannelDown.to_command_request(expected.id);

    assert_command_request(result, expected);
}

#[test]
fn test_get_external_input_list() {
    let expected = CommandRequest {
        id: 0,
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/getExternalInputList"),
        payload: None,
    };

    let result = commands::tv::GetExternalInputList.to_command_request(expected.id);

    assert_command_request(result, expected);
}

use crate::lg_command_request::assert_command_request;
use lg_webos_client::lg_command::request_commands;
use lg_webos_client::lg_command::REQUEST_TYPE;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest};
use serde_json::json;

#[test]
fn test_set_open_channel() {
    let open_channel_id = "1_21_6_1_1519_48608_1519".to_string();
    let expected = CommandRequest {
        r#type: REQUEST_TYPE.to_string(),
        uri: String::from("ssap://tv/openChannel"),
        payload: Some(json!({ "channelId": open_channel_id.clone() })),
    };

    let result = request_commands::tv::SetOpenChannel {
        channel_id: open_channel_id,
    }
    .to_command_request();

    assert_command_request(result, expected);
}

#[test]
fn test_switch_input() {
    for input_id in ["test", "123", "testing"] {
        let input_id = input_id.to_string();

        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/switchInput"),
            payload: Some(json!({ "inputId":input_id.clone() })),
        };

        let result = request_commands::tv::SwitchInput { input_id }.to_command_request();

        assert_command_request(result, expected);
    }
}

#[test]
fn test_get_open_channel_information() {
    for channel_id in ["Hello", "My", "Test"] {
        let channel_id = channel_id.to_string();
        let expected = CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/openChannel"),
            payload: Some(json!({ "channelId": channel_id.clone() })),
        };

        let result =
            request_commands::tv::GetOpenChannelInformation { channel_id }.to_command_request();

        assert_command_request(result, expected);
    }
}

#[test]
fn no_payload_commands() {
    let expected_requests = vec![
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getChannelList"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getCurrentChannel"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/channelUp"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/channelDown"),
            payload: None,
        },
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://tv/getExternalInputList"),
            payload: None,
        },
    ];

    let commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::tv::GetChannelList),
        Box::new(request_commands::tv::GetCurrentChannelInformation),
        Box::new(request_commands::tv::ChannelUp),
        Box::new(request_commands::tv::ChannelDown),
        Box::new(request_commands::tv::GetExternalInputList),
    ];

    for (command, expected_request) in commands.iter().zip(expected_requests) {
        assert_command_request(command.to_command_request(), expected_request);
    }
}

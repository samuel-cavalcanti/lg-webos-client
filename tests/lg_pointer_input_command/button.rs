use lg_webos_client::lg_command::pointer_input_commands::{self, PointerInputCommand};

#[test]
fn test_buttons() {
    /*
        After testing in TV the correct payload what the client must send. This test is was created
    */
    let input_commands: Vec<Box<dyn PointerInputCommand>> = vec![
        Box::new(pointer_input_commands::ButtonKey::DASH),
        Box::new(pointer_input_commands::ButtonKey::GUIDE),
        Box::new(pointer_input_commands::ButtonKey::HOME),
        Box::new(pointer_input_commands::ButtonKey::LEFT),
        Box::new(pointer_input_commands::ButtonKey::RIGHT),
        Box::new(pointer_input_commands::ButtonKey::UP),
        Box::new(pointer_input_commands::ButtonKey::DOWN),
        Box::new(pointer_input_commands::ButtonKey::BACK),
        Box::new(pointer_input_commands::ButtonKey::ENTER),
        // numbers
        Box::new(pointer_input_commands::ButtonKey::Num0),
        Box::new(pointer_input_commands::ButtonKey::Num1),
        Box::new(pointer_input_commands::ButtonKey::Num2),
        Box::new(pointer_input_commands::ButtonKey::Num3),
        Box::new(pointer_input_commands::ButtonKey::Num4),
        Box::new(pointer_input_commands::ButtonKey::Num5),
        Box::new(pointer_input_commands::ButtonKey::Num6),
        Box::new(pointer_input_commands::ButtonKey::Num7),
        Box::new(pointer_input_commands::ButtonKey::Num8),
        Box::new(pointer_input_commands::ButtonKey::Num9),
    ];

    let mut expected_numbers = (0..10).map(|i| format!("{i}")).collect::<Vec<String>>();

    let expected_strings = vec![
        "DASH".to_string(),
        "GUIDE".to_string(),
        "HOME".to_string(),
        "LEFT".to_string(),
        "RIGHT".to_string(),
        "UP".to_string(),
        "DOWN".to_string(),
        "BACK".to_string(),
        "ENTER".to_string(),
    ];

    let mut expected = expected_strings;

    expected.append(&mut expected_numbers);

    let expected_command_string: Vec<String> = expected
        .into_iter()
        .map(|name| format!("type:button\nname:{name}\n\n"))
        .collect();

    for (command, expected_string) in input_commands.iter().zip(expected_command_string) {
        assert_eq!(command.to_request_string(), expected_string);
    }
}

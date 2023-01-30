use super::PointerInputCommand;

pub enum ButtonKey {
    HOME,
    BACK,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ENTER,
}

impl PointerInputCommand for ButtonKey {
    fn to_request_string(&self) -> String {
        let name = match self {
            ButtonKey::ENTER => "ENTER",
            ButtonKey::HOME => "HOME",
            ButtonKey::LEFT => "LEFT",
            ButtonKey::RIGHT => "RIGHT",
            ButtonKey::UP => "UP",
            ButtonKey::DOWN => "DOWN",
            ButtonKey::BACK => "BACK",
        };

        format!("type:button\nname:{name}\n\n")
    }
}

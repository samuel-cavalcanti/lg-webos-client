use super::PointerInputCommand;

pub enum Button {
    HOME,
    BACK,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ENTER,
}

impl PointerInputCommand for Button {
    fn to_string(&self) -> String {
        let name = match self {
            Button::ENTER => "ENTER",
            Button::HOME => "HOME",
            Button::LEFT => "LEFT",
            Button::RIGHT => "RIGHT",
            Button::UP => "UP",
            Button::DOWN => "DOWN",
            Button::BACK => "BACK",
        };

        return format!("type:button\nname:{name}\n\n");
    }
}

use super::PointerInputCommand;

pub enum ButtonKey {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    HOME,
    BACK,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ENTER,
    DASH,
    GUIDE,
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
            ButtonKey::DASH => "DASH",
            ButtonKey::GUIDE => "GUIDE",

            ButtonKey::Num0 => "0",
            ButtonKey::Num1 => "1",
            ButtonKey::Num2 => "2",
            ButtonKey::Num3 => "3",
            ButtonKey::Num4 => "4",
            ButtonKey::Num5 => "5",
            ButtonKey::Num6 => "6",
            ButtonKey::Num7 => "7",
            ButtonKey::Num8 => "8",
            ButtonKey::Num9 => "9",
        };

        format!("type:button\nname:{name}\n\n")
    }
}

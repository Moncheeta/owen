pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent)
}

pub enum Key {
    // Character
    Char(char),

    // Navigation Keys
    Up,
    Down,
    Left,
    Right,

    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,

    // Function Keys
    Function(u8)
}

pub enum Modifier {
    Ctrl,
    Alt
}

pub enum KeyEventType {
    Press,
    Release
}

// TODO: switch modifiers to bitflags
pub struct KeyEvent {
    key: Key,
    modifiers: Vec<Modifier>,
    r#type: KeyEventType
}

pub enum MouseButton {
    Left,
    Middle,
    Right
}

pub enum MouseEvent {
    Press(MouseButton),
    Release(MouseButton)
}

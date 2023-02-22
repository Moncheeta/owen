pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent)
}

pub enum Key {
    // Character
    Char(char),

    // Modifiers
    Ctrl(Box<Key>),
    Alt(Box<Key>),

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

pub enum KeyEvent {
    Press(Key),
    Release(Key)
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

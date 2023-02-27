// Represents a character in the terminal
#[derive(Clone, Copy)]
pub struct Cell {
    content: Option<char>
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            content: None
        }
    }
}


// Represents a character in the terminal
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub symbol: char,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { symbol: ' ' }
    }
}

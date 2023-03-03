#[derive(Copy, Clone)]
pub struct Size {
    // Vertical Size
    pub rows: u16,
    // Horizontal Size
    pub columns: u16,
}

impl Size {
    pub fn new(rows: u16, columns: u16) -> Self {
        Size { rows, columns }
    }

    pub fn area(&self) -> u16 {
        self.rows * self.columns
    }
}

// The origin is at the top left of the terminal
#[derive(Copy, Clone)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

impl Position {
    pub fn new(row: u16, column: u16) -> Self {
        Position { row, column }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TODO: Write contraints
struct Arragement {
    pub direction: Direction,
}

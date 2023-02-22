#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size {
            width,
            height
        }
    }

    pub fn area(&self) -> u16 {
        self.width * self.height
    }
}

impl Default for Size {
    fn default() -> Self {
        Size {
            width: 0,
            height: 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub row: u16,
    pub column: u16
}

impl Position {
    pub fn new(row: u16, column: u16) -> Self {
        Position {
            row,
            column
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            row: 0,
            column: 0
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Arragement {
    pub direction: Direction,
//    constraints: Vec<Constraint>
}

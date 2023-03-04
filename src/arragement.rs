#[derive(Copy, Clone)]
pub struct Size {
    // Vertical Size
    pub rows: u16,
    // Horizontal Size
    pub columns: u16,
}

impl Size {
    pub fn area(&self) -> u16 {
        self.rows * self.columns
    }
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Size {
            rows: value.0,
            columns: value.1,
        }
    }
}

// The origin is at the top left of the terminal
#[derive(Copy, Clone)]
pub struct Position {
    pub row: u16,
    pub column: u16,
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Position {
            row: value.0,
            column: value.1,
        }
    }
}

// TODO: Write arragment and contraints

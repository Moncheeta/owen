#[derive(Copy, Clone)]
// A structure to organize a position of an object on the terminal.
// The row and column start at 0. The origin is the top left corner of the terminal;
pub struct Position {
    // The amount of rows from the top of the terminal
    pub row: u16,
    // The amount of columns from the left side of the terminal.
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

#[derive(Copy, Clone)]
// A structure to organize a size of an object on the terminal.
// The amount of rows and columns starts at 1.
pub struct Size {
    // Vertical Size
    pub rows: u16,
    // Horizontal Size
    pub columns: u16,
}

impl Size {
    // The amount of space that the object takes up
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

// TODO: Write arragment and contraints

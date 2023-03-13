use crate::arragement::{Position, Size};

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

pub struct Buffer {
    content: Vec<Cell>,
    size: Size,
}

impl Buffer {
    // Creates a new buffer with a certain size
    pub fn new(size: Size) -> Buffer {
        Buffer {
            content: vec![Cell::default(); size.area() as usize],
            size,
        }
    }

    // Gets the size of the buffer
    pub fn size(&self) -> Size {
        self.size
    }

    // Gets the index of the position in self.content
    fn index_of(&self, position: Position) -> usize {
        ((position.row * self.size.columns) + position.column).into()
    }

    // Gets the cell at a position
    pub fn cell(&self, position: Position) -> Option<&Cell> {
        let index = self.index_of(position);
        if index > self.content.len() {
            return None;
        }
        Some(&self.content[index])
    }

    // Gets a mutable reference to a cell at a position
    pub fn cell_mut(&mut self, position: Position) -> Option<&mut Cell> {
        let index = self.index_of(position);
        if index > self.content.len() {
            return None;
        }
        Some(&mut self.content[index])
    }

    // Resizes the frame to a new size
    pub fn resize(&mut self, size: Size) {
        self.content
            .resize_with(size.area().into(), Default::default);
        self.size = size;
    }

    // Finds the difference between itself and another frame
    // Returns all the cells that are different from itself
    pub fn diff<'b>(&self, other: &'b Buffer) -> Vec<(&'b Cell, Position)> {
        let mut diff = Vec::new();
        for row in 0..self.size.rows {
            for column in 0..self.size.columns {
                let position = (row, column).into();
                match (other.cell(position), self.cell(position)) {
                    (Some(other_cell), Some(buffer_cell)) => {
                        if other_cell != buffer_cell {
                            diff.push((other_cell, position));
                        }
                    }
                    (Some(other_cell), None) => diff.push((other_cell, position)),
                    _ => (),
                }
            }
        }
        diff
    }
}

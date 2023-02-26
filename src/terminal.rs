use std::io;
use crate::arragement::{Size, Position};
use crate::backend::Backend;

// Represents a character in the terminal
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

pub struct Frame {
    buffer: Vec<Cell>,
    size: Size
}

impl Frame {
    fn new(size: Size) -> Self {
        Frame {
            buffer: Vec::new(),
            size
        }
    }

    // Gets the Cell at a position
    fn get(&self, position: Position) -> &Cell {
        let index = self.index_of(position.row, position.column);
        &self.buffer[index]
    }

    // Gets a mutable reference to a Cell at a position
    fn get_mut(&mut self, position: Position) -> &mut Cell {
        let index = self.index_of(position.row, position.column);
        &mut self.buffer[index]
    }

    // Returns the index of a Cell at a position
    fn index_of(&self, row: u16, column: u16) -> usize {
        ((row * self.size.width) + column).into()
    }

    // Resizes the frame to a new size
    fn resize(&mut self, size: Size) {
        self.buffer.resize_with(size.area().into(), Default::default);
    }
}

pub struct Terminal<B>
where
    B: Backend
{
    backend: B,
    frames: [Frame; 2],
}

impl<B> Terminal<B>
where
    B: Backend
{
    fn new(backend: B) -> io::Result<Terminal<B>> {
        let size = backend.size()?;
        Ok(Terminal {
            backend,
            frames: [
                Frame::new(size),
                Frame::new(size)
            ]
        })
    }


}

use std::io::{
    Write,
    Error
};
use crossterm::{
    queue,
    terminal::{
        size,
        Clear,
        ClearType
    },
    cursor::{
        position,
        Show,
        Hide,
        MoveTo
    }
};
use crate::{
    frame::Frame,
    arragement::{
        Size,
        Position
    }
};

pub struct Terminal<W>
where
    W: Write,
{
    // Output stream
    buffer: W,
    frames: [Frame; 2],
    // The frame is to be drawn
    index: u8
}

impl<W> Terminal<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> Result<Terminal<W>, Error> {
        let size = match size() {
            Ok((columns, rows)) => Size { rows, columns },
            Err(err) => return Err(err)
        };
        Ok(Terminal {
            buffer,
            frames: [
                Frame::new(size),
                Frame::new(size)
            ],
            index: 0
        })
    }

    // Switches the current frame index
    fn switch(&mut self) {
        if self.index == 1 {
            self.index = 0;
        } else {
            self.index = 1;
        }
    }

    // TODO: Write drawing routine
    // Draws the current frame onto the terminal
    fn draw(&mut self) -> Result<(), Error> {
        todo!()
    }

    // Clears the terminal
    fn clear(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Clear(ClearType::All))
    }

    // Returns the size of the terminal
    fn size() -> Result<Size, Error> {
        match size() {
            Ok((columns, rows)) => Ok(Size {
                rows,
                columns
            }),
            Err(err) => Err(err)
        }
    }

    // Hides the cursor in the terminal
    fn cursor_hide(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Hide)
    }

    // Shows the cursor in the terminal
    fn cursor_show(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Show)
    }

    // Gets the position of the cursor in the terminal
    fn cursor_get() -> Result<Position, Error> {
        let cursor_position = position()?;
        Ok(Position {
            row: cursor_position.1,
            column: cursor_position.0
        })
    }

    // Changes the position of the cursor in the terminal
    fn cursor_set(&mut self, position: Position) -> Result<(), Error> {
        queue!(self.buffer, MoveTo(position.column, position.row))
    }
}

use crate::{
    arragement::{Position, Size},
    frame::Frame,
};
use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    execute, queue,
    style::Print,
    terminal::{size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{Error, Write};

pub struct Terminal<W>
where
    W: Write,
{
    // Output stream
    buffer: W,
    // The difference between the two is updated in the terminal
    frames: [Frame; 2],
    // The frame that is drawn
    index: usize,
}

impl<W> Terminal<W>
where
    W: Write,
{
    pub fn new(buffer: W) -> Result<Terminal<W>, Error> {
        let size = Terminal::<W>::size()?;
        Ok(Terminal {
            buffer,
            frames: [Frame::new(size), Frame::new(size)],
            index: 0,
        })
    }

    // Switches the current frame index
    fn switch(&mut self) {
        if self.index == 1 {
            self.index = 0;
        } else {
            self.index = 1;
        };
    }

    // Gets the size of the terminal
    pub fn size() -> Result<Size, Error> {
        match size() {
            Ok((columns, rows)) => Ok(Size { rows, columns }),
            Err(err) => Err(err),
        }
    }

    // Makes the terminal switch to an alternate screen
    pub fn alternate_screen_enter(&mut self) -> Result<(), Error> {
        execute!(self.buffer, EnterAlternateScreen)
    }

    // Makes the terminal leave the alternate screen
    pub fn alternate_screen_leave(&mut self) -> Result<(), Error> {
        execute!(self.buffer, LeaveAlternateScreen)
    }

    // Draws the current frame onto the terminal
    pub fn draw(&mut self, renderer: impl FnOnce(&mut Frame)) -> Result<(), Error> {
        renderer(&mut self.frames[1 - self.index]);
        let mut cursor_position: Position = match position() {
            Ok((column, row)) => (row, column).into(),
            Err(err) => return Err(err),
        };
        // The difference between the current frame and the previous frame
        let diff = self.frames[self.index]
            .buffer
            .diff(&self.frames[1 - self.index].buffer);
        // Update the cells on the screen from the difference
        for (cell, position) in diff {
            if position.row != cursor_position.row
                || position.column as i32 != cursor_position.column as i32 - 1
            {
                queue!(self.buffer, MoveTo(position.column, position.row))?;
                cursor_position.row = position.row;
                cursor_position.column = position.column;
            } else {
                cursor_position.column += 1;
            }
            queue!(self.buffer, Print(cell.symbol))?;
            self.buffer.flush()?;
        }
        // Send all commands to the terminal
        self.buffer.flush()?;
        self.switch();
        Ok(())
    }

    // Hides the cursor in the terminal
    pub fn cursor_hide(&mut self) -> Result<(), Error> {
        execute!(self.buffer, Hide)
    }

    // Shows the cursor in the terminal
    pub fn cursor_show(&mut self) -> Result<(), Error> {
        execute!(self.buffer, Show)
    }

    // Gets the position of the cursor in the terminal
    pub fn cursor_get() -> Result<Position, Error> {
        let cursor_position = position()?;
        Ok(Position {
            row: cursor_position.1,
            column: cursor_position.0,
        })
    }

    // Changes the position of the cursor in the terminal
    pub fn cursor_move(&mut self, position: Position) -> Result<(), Error> {
        execute!(self.buffer, MoveTo(position.column, position.row))
    }
}

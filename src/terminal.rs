use crate::{
    arragement::{
        Position,
        // Position
        Size,
    },
    frame::Frame,
};
use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    queue,
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
    // The frame is to be drawn
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
    fn switch(&mut self) -> usize {
        if self.index == 1 {
            self.index = 0;
        } else {
            self.index = 1;
        };
        self.index
    }

    // Draws the current frame onto the terminal
    pub fn draw(&mut self, renderer: impl FnOnce(&mut Frame)) -> Result<(), Error> {
        renderer(&mut self.frames[self.index]);
        let mut cursor_position = Terminal::<W>::cursor_get()?;
        for (cell, position) in self.frames[self.index].diff(&self.frames[1 - self.index]) {
            if position.row != cursor_position.row || position.column != cursor_position.column - 1
            {
                self.cursor_move(position)?;
            } else {
                cursor_position.column += 1;
            }
            queue!(self.buffer, Print(cell.symbol))?;
        }
        self.switch();
        Ok(())
    }

    // Gets the size of the terminal
    pub fn size() -> Result<Size, Error> {
        match size() {
            Ok((columns, rows)) => Ok(Size { rows, columns }),
            Err(err) => Err(err),
        }
    }

    // Makes the terminal switch to an alternate screen
    pub fn enter_alternate_screen(&mut self) -> Result<(), Error> {
        queue!(self.buffer, EnterAlternateScreen)
    }

    // Makes the terminal leave the alternate screen
    pub fn leave_alternate_screen(&mut self) -> Result<(), Error> {
        queue!(self.buffer, LeaveAlternateScreen)
    }

    // Hides the cursor in the terminal
    pub fn cursor_hide(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Hide)
    }

    // Shows the cursor in the terminal
    pub fn cursor_show(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Show)
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
        queue!(self.buffer, MoveTo(position.column, position.row))
    }
}

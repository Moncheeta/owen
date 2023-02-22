pub mod event;
use std::io;
use crate::arragement::{Size, Position};
use crate::backend::event::Event;
use crate::terminal::Cell;

pub trait Backend {
    // Terminal events
    fn events() -> Result<Event, io::Error>;

    // Draws each cell in an iterator at a position in the terminal
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
        where
            I: Iterator<Item = (Position, &'a Cell)>;
    // Clears the terminal's screen buffer
    fn clear(&mut self) -> Result<(), io::Error>;
    // Flushes the output stream
    fn flush(&mut self) -> Result<(), io::Error>;

    // Gets the size of the terminal
    fn size(&self) -> Result<Size, io::Error>;
    // Gets the size of the terminal in pixels
    fn size_pixels(&self) -> Result<Size, io::Error>;

    // Whether to show the cursor or not
    fn cursor(&self, state: bool) -> Result<(), io::Error>;
    // Gets the position of the cursor
    fn get_cursor(&self) -> Result<Position, io::Error>;
    // Sets the position of the cursor
    fn set_cursor(&self, position: Position) -> Result<(), io::Error>;
}


pub mod crossterm;
use std::{
    io::Error,
    time::Duration
};
use crate::{
    arragement::{Size, Position},
    event::Event,
    terminal::Cell
};

pub trait Backend {
    // Returns a boolean of whether there is an event available
    fn poll(&self, timeout: Duration) -> Result<bool, Error>;
    // Waits until an event is available
    fn read(&self) -> Result<Event, Error>;
    // Reads all events that are available
    fn read_all(&self) -> Result<Vec<Event>, Error>;
    // Draws each cell in an iterator at a position in the terminal
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error>
        where
            I: Iterator<Item = (Position, &'a Cell)>;
    // Clears the terminal's screen buffer
    fn clear(&mut self) -> Result<(), Error>;
    // Gets the size of the terminal
    fn size(&self) -> Result<Size, Error>;
    // Hides the cursor in the terminal
    fn cursor_hide(&mut self) -> Result<(), Error>;
    // Shows the cursor in the terminal
    fn cursor_show(&mut self) -> Result<(), Error>;
    // Gets the position of the cursor
    fn cursor_get(&self) -> Result<Position, Error>;
    // Sets the position of the cursor
    fn cursor_set(&mut self, position: Position) -> Result<(), Error>;
}


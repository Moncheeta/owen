use std::{
    io::{Write, Error, ErrorKind},
    time::Duration
};
use crossterm::{
    queue,
    terminal::{size, Clear, ClearType},
    cursor::{position, Show, Hide, MoveTo},
    event::{Event, poll, read}
};
use crate::{
    backend::Backend,
    arragement::{Size, Position},
};

struct CrosstermBackend<W: Write> {
    buffer: W
}

impl<W> CrosstermBackend<W>
where
    W: Write,
{
    fn new(buffer: W) -> CrosstermBackend<W> {
        CrosstermBackend {
            buffer
        }
    }
}

impl<W> Write for CrosstermBackend<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

impl<W> Backend for CrosstermBackend<W>
where
    W: Write,
{
    fn poll(&self, timeout: Duration) -> Result<bool, Error> {
        poll(timeout)
    }

    fn read(&self) -> Result<crate::event::Event, Error> {
        match read() {
            Ok(event) => event.try_into(),
            Err(err) => Err(err)
        }
    }

    fn read_all(&self) -> Result<Vec<crate::event::Event>, Error> {
        let mut events = Vec::new();
        loop {
            match self.poll(Duration::ZERO) {
                Ok(true) => {
                    match self.read() {
                        Ok(event) => events.push(event),
                        Err(err) => return Err(err)
                    };
                },
                Ok(false) => break,
                Err(err) => return Err(err)
            };
        }
        Ok(events)
    }

    // TODO: Write drawing routine
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error>
            where
                I: Iterator<Item = (crate::arragement::Position, &'a crate::terminal::Cell)> {
        todo!()
    }

    fn clear(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Clear(ClearType::All))
    }

    fn size(&self) -> Result<crate::arragement::Size, Error> {
        match size() {
            Ok((width, height)) => Ok(Size {
                width,
                height
            }),
            Err(err) => Err(err)
        }
    }

    fn cursor_hide(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Hide)
    }

    fn cursor_show(&mut self) -> Result<(), Error> {
        queue!(self.buffer, Show)
    }

    fn cursor_get(&self) -> Result<crate::arragement::Position, Error> {
        let cursor_position = position()?;
        Ok(Position {
            row: cursor_position.1,
            column: cursor_position.0
        })
    }

    fn cursor_set(&mut self, position: crate::arragement::Position) -> Result<(), Error> {
        queue!(self.buffer, MoveTo(position.column, position.row))
    }

}

impl TryFrom<Event> for crate::event::Event {
    type Error = Error;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        // TODO: Write complete conversion
        match value {
            _ => Err(Error::from(ErrorKind::Unsupported)),
        }
    }
}

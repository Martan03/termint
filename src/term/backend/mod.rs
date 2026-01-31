use std::time::Duration;

#[cfg(feature = "backend-crossterm")]
mod crossterm;
mod event;
#[cfg(feature = "backend-termal")]
mod termal;

#[cfg(feature = "backend-crossterm")]
pub use crossterm::CrosstermBackend;
pub use event::Event;
#[cfg(feature = "backend-termal")]
pub use termal::TermalBackend;

use crate::Error;

pub trait Backend {
    /// Polls for an event and returns it if available within the timeout
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error>;
}

pub struct NoBackend;

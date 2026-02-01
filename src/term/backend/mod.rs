use std::time::Duration;

#[cfg(feature = "backend-crossterm")]
mod crossterm;
mod event;
#[cfg(feature = "backend-termal")]
mod termal;

#[cfg(feature = "backend-crossterm")]
pub use crossterm::CrosstermBackend;
pub use event::*;
#[cfg(feature = "backend-termal")]
pub use termal::TermalBackend;

use crate::Error;

#[cfg(feature = "backend-termal")]
pub type DefaultBackend = TermalBackend;

#[cfg(all(not(feature = "backend-termal"), feature = "backend-crossterm"))]
pub type DefaultBackend = CrosstermBackend;

#[cfg(all(
    not(feature = "backend-termal"),
    not(feature = "backend-crossterm")
))]
pub type DefaultBackend = NoBackend;

pub trait Backend: Default {
    /// Polls for an event and returns it if available within the timeout
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error>;
}

#[derive(Debug, Default)]
pub struct NoBackend;

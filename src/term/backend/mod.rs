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

/// Backend trait allows creating custom backends, which then can be used as
/// a custom [`Term`] backend in the Framework mode.
pub trait Backend: Default {
    /// Polls for an event and returns it if available within the timeout
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error>;
}

/// This is used when no backend feature is enabled. When [`Term`] has
/// `NoBackend`, the Framework mode is disabled.
///
/// # Example
///
/// If you want to use the `NoBackend` with [`Term], you can create it like
/// this:
///
/// ```rust
/// use termint::prelude::*;
/// use termint::term::backend::NoBackend;
///
/// Term::<NoBackend>::new();
/// ```
#[derive(Debug, Default)]
pub struct NoBackend;

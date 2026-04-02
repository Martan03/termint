//! Terminal event-reading backend trait and implementations
//!
//! # Backend implementations
//!
//! Backend implements reading terminal events and is mainly used in
//! combination of [`Term`](crate::term::Term).
//!
//! Currently two backends are supported:
//! - [`CrosstermBackend`] - requires
//!   `backend-crossterm` feature (default).
//! - [`TermalBackend`] - requires
//!   `backend-termal` feature.
//!
//! You can create new [`Term`](crate::term::Term) with specified backend like
//! this:
//! ```rust,no_run
//! # fn main() {
//! # #[cfg(feature = "backend-crossterm")]
//! # {
//! use termint::prelude::*;
//!
//! let term = Term::<(), CrosstermBackend>::new();
//! # }
//! # }
//! ```
//!
//! # Backend trait
//!
//! This trait is can be used to implement a custom backend.
//!
//! # Example
//!
//! Reading events from the given backend:
//!
//! ```rust
//! use std::{io::Write, time::Duration};
//! use termint::{
//!     prelude::*,
//!     term::{
//!         backend::{Backend, DefaultBackend},
//!         enable_bracketed_paste, enable_mouse_capture,
//!         disable_bracketed_paste, disable_mouse_capture
//!     }
//! };
//!
//! fn print_events<B: Backend>(mut backend: B) -> Result<(), Error> {
//!     DefaultBackend::enable_raw_mode()?;
//!     enable_bracketed_paste();
//!     enable_mouse_capture();
//!
//!     let mut stdout = std::io::stdout();
//!     let mut timeout = Duration::from_millis(100);
//!     loop {
//!         if let Some(event) = backend.read_event(timeout)? {
//!             print!("{:?}\n\r", event);
//!             _ = stdout.flush();
//!         }
//!     }
//!
//!     disable_bracketed_paste();
//!     disable_mouse_capture();
//!     DefaultBackend::disable_raw_mode()?;
//!     Ok(())
//! }
//! ```

use std::time::Duration;

#[cfg(feature = "backend-crossterm")]
mod crossterm;
mod event;
#[cfg(feature = "backend-termal")]
mod termal;
mod test;

#[cfg(feature = "backend-crossterm")]
pub use crossterm::CrosstermBackend;
pub use event::*;
#[cfg(feature = "backend-termal")]
pub use termal::TermalBackend;
pub use test::TestBackend;

use crate::Error;

#[cfg(feature = "backend-crossterm")]
pub type DefaultBackend = CrosstermBackend;

#[cfg(all(not(feature = "backend-crossterm"), feature = "backend-termal"))]
pub type DefaultBackend = TermalBackend;

#[cfg(all(
    not(feature = "backend-termal"),
    not(feature = "backend-crossterm")
))]
pub type DefaultBackend = TestBackend;

/// Backend trait allows creating custom backends, which then can be used as
/// a custom [`Term`](crate::term::Term) backend in the Framework mode.
pub trait Backend: Default {
    /// Polls for an event and returns it if available within the timeout
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error>;

    /// Enables terminal raw mode.
    fn enable_raw_mode() -> Result<(), Error>;

    /// Disables terminal raw mode.
    fn disable_raw_mode() -> Result<(), Error>;

    /// Checks if terminal raw mode is enabled.
    fn is_raw_mode_enabled() -> bool;

    /// Gets the size of the terminal.
    fn get_size(&self) -> Result<(usize, usize), Error>;
}

//! Terminal event-reading backend trait and implementations
//!
//! # Backend implementations
//!
//! Backend implements reading terminal events and is mainly used in
//! combination of [`crate::term::Term`].
//!
//! Currently two backends are supported:
//! - [`CrosstermBackend`] - requires `backend-crossterm` feature (default).
//! - [`TermalBackend`] - requires `backend-termal` feature.
//!
//! You can create new [`crate::term::Term`] with specified backend like this:
//! ```rust
//! use termint::prelude::*;
//!
//! let term = Term::<(), TermalBackend>::new();
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
//!         backend::Backend, enable_bracketed_paste, enable_mouse_capture,
//!         disable_bracketed_paste, disable_mouse_capture
//!     }
//! };
//! use termal::raw::{disable_raw_mode, enable_raw_mode};
//!
//! fn print_events<B: Backend>(mut backend: B) -> Result<(), Error> {
//!     enable_raw_mode()?;
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
//!     disable_raw_mode()?;
//!     Ok(())
//! }
//! ```

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
/// a custom [`crate::term::Term`] backend in the Framework mode.
pub trait Backend: Default {
    /// Polls for an event and returns it if available within the timeout
    fn read_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<Event>, Error>;
}

/// This is used when no backend feature is enabled. When [`crate::term::Term`]
/// has `NoBackend`, the Framework mode is disabled.
///
/// # Example
///
/// If you want to use the `NoBackend` with [`crate::term::Term], you can
/// create it like this:
///
/// ```rust
/// use termint::prelude::*;
/// use termint::term::backend::NoBackend;
///
/// Term::<NoBackend>::new();
/// ```
#[derive(Debug, Default)]
pub struct NoBackend;

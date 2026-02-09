//! Core terminal abstraction and application lifecycle.
//!
//! This module provides the [`Application`] trait, which allows building
//! stateful apps easily. The [`Term`](crate::term::Term) is also provided by
//! this module and provides rendering options together with running the
//! provided app implementation and handling the basic lifecycle.
//!
//! # Backend
//!
//! This module also contains backend module, which provides
//! [`Event`](crate::term::backend::Event) enum,
//! [`Backend`](crate::term::backend::Backend) trait and backend
//! implementations - currently those are
//! [`CrosstermBackend`](crate::term::backend::CrosstermBackend) and
//! [`TermalBackend`](crate::term::backend::TermalBackend) if you have the
//! corresponding features enabled (`backend-crossterm` or `backend-termal`).
//!
//! # Example
//!
//! Implementing [`Application`] and running it in Framework mode using
//! [`Term::run`](crate::term::Term::run).
//!
//! ```rust,no_run
//! use termint::prelude::*;
//!
//! struct MyApp;
//!
//! impl Application for MyApp {
//!     type Message = ();
//!
//!     fn view(&self, _frame: &Frame) -> Element<Self::Message> {
//!         let mut main = Block::vertical().title("Termint App");
//!         main.push("Hello from the Application trait!".fg(Color::Cyan), 0..);
//!         main.into()
//!     }
//!
//!     fn event(&mut self, event: Event) -> Action {
//!         match event {
//!             Event::Key(k) if k.code == KeyCode::Char('q') => Action::QUIT,
//!             _ => Action::NONE,
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), Error> {
//!     Term::default().setup()?.run(&mut MyApp)
//! }
//! ```
//!
//! [`Application`]: crate::term::Application

mod action;
mod app;
pub mod backend;
mod frame;
#[allow(clippy::module_inception)]
mod term;

use std::io::{stdout, Write};

pub use action::Action;
pub use app::Application;
pub use frame::Frame;
pub use term::Term;
use termal::codes::{
    DISABLE_BRACKETED_PASTE_MODE, DISABLE_MOUSE_XY_ALL_TRACKING,
    DISABLE_MOUSE_XY_EXT, ENABLE_BRACKETED_PASTE_MODE,
    ENABLE_MOUSE_XY_ALL_TRACKING, ENABLE_MOUSE_XY_EXT,
};

/// Enables mouse events backend capture.
pub fn enable_mouse_capture() {
    print!("{}{}", ENABLE_MOUSE_XY_ALL_TRACKING, ENABLE_MOUSE_XY_EXT);
    _ = stdout().flush();
}

/// Disable mouse events backend capture.
pub fn disable_mouse_capture() {
    print!("{}{}", DISABLE_MOUSE_XY_ALL_TRACKING, DISABLE_MOUSE_XY_EXT);
    _ = stdout().flush();
}

/// Enables bracketed paste mode, which allows capturing `Event::Paste`.
pub fn enable_bracketed_paste() {
    print!("{}", ENABLE_BRACKETED_PASTE_MODE);
    _ = stdout().flush();
}

/// Disables bracketed paste mode.
pub fn disable_bracketed_paste() {
    print!("{}", DISABLE_BRACKETED_PASTE_MODE);
    _ = stdout().flush();
}

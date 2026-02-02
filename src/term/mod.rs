//! Core terminal abstraction and application lifecycle.
//!
//! This module provides the [`Application`] trait, which allows building
//! stateful apps easily. The [`Term`] is also provided by this module and
//! provides rendering options together with running the provided app
//! implementation and handling the basic lifecycle.

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

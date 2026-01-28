//! Core terminal abstraction and application lifecycle.
//!
//! This module provides the [`Application`] trait, which allows building
//! stateful apps easily. The [`Term`] is also provided by this module and
//! provides rendering options together with running the provided app
//! implementation and handling the basic lifecycle.

mod action;
mod app;
mod frame;
#[allow(clippy::module_inception)]
mod term;

pub use action::Action;
pub use app::Application;
pub use frame::Frame;
pub use term::Term;

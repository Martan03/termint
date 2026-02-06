//! A collection of types that implement the [`Widget`] trait.
//!
//! This module provides a variety of TUI components (widgets) used for
//! rendering.
//!
//! # Available widgets:
//! - [`BgGrad`]: A container widget that renders a gradient background behind
//! its child widget.
//! - [`Block`]: A widget that wrap another widget and adds border and title.
//! - [`Grad`]: A widget for rendering text with a gradient foreground color.
//! - [`Grid`]: A layout widget that arranges children in a grid specified by
//! rows and columns.
//! - [`Layout`]: A container widget that arranges child widgets in a single
//! direction, flexing their sizes based on given constraints.
//! - [`List`]: A scrollable list widget with suuport for item selection and
//! highlighting.
//! - [`Overlay`]: A widget that stacks its children in layers, from bottom to
//! top.
//! - [`Paragraph`]: A widget combining multiple widgets implementing the
//! [`Text`] trait into single widget.
//! - [`ProgressBar`]: A widget that displays a horizontal progress bar.
//! - [`Scrollable`]: A wrapper widget that adds scrollability to its child
//! when content overflows.
//! - [`Scrollbar`]: A scrollbar widget that can be either vertical or
//! horizontal.
//! - [`Spacer`]: A spacer widget used for layout spacing.
//! - [`Span`]: A widget for styling text where all characters share the same
//! style.
//! - [`Table`]: A widget that displays a table with configurable column
//! widths, optional header and scrollable row content.

mod bg_grad;
mod block;
mod button;
mod grad;
mod grid;
mod layout;
mod list;
mod overlay;
mod paragraph;
mod progress_bar;
mod scrollable;
mod scrollbar;
mod spacer;
mod span;
mod table;
mod widget;

pub mod cache;

pub use bg_grad::BgGrad;
pub use block::Block;
pub use button::Button;
pub use grad::Grad;
pub use grid::Grid;
pub use layout::Layout;
pub use list::{List, ListState};
pub use overlay::Overlay;
pub use paragraph::Paragraph;
pub use progress_bar::ProgressBar;
pub use scrollable::Scrollable;
pub use scrollbar::{Scrollbar, ScrollbarState};
pub use spacer::Spacer;
pub use span::{Span, ToSpan};
pub use table::{Row, Table, TableState};
pub use widget::{Element, EventResult, Widget};

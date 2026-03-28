//! A collection of types that implement the [`Widget`](crate::widgets::Widget)
//! trait.
//!
//! This module provides a variety of TUI components (widgets) used for
//! rendering.
//!
//! # Available widgets:
//!
//! The widgets are categorized by their primary function:
//!
//! ## Layouts
//!
//! Widgets that organize other widgets.
//!
//! - [`Layout`](crate::widgets::Layout): Flex-box style arrangement
//!   (row/column).
//! - [`Grid`](crate::widgets::Grid): Align children in rows and columns.
//! - [`Overlay`](crate::widgets::Overlay): Stack widgets on top of each other.
//! - [`Spacer`](crate::widgets::Spacer): Empty widget for layout padding.
//!
//! ## Containers
//!
//! Widgets that wrap other widgets and add some functionality/style.
//!
//! - [`Block`](crate::widgets::Block): Wraps a widget and adds border and
//!   title.
//! - [`Button`](crate::widgets::Button): Wraps a widget and adds mouse click
//!   handling.
//! - [`Scrollable`](crate::widgets::Scrollable): Adds scrollbars to
//!   overflowing content.
//! - [`BgGrad`](crate::widgets::BgGrad): Wraps a widget and adds a background
//!   gradient.
//!
//! ## Text
//!
//! Widgets for displaying text.
//!
//! - [`Span`](crate::widgets::Span): Styled text string.
//! - [`Grad`](crate::widgets::Grad): Gradient-colored text.
//! - [`Paragraph`](crate::widgets::Paragraph): Combines multiple styled texts
//!   into one widget.
//!
//! ## Data
//!
//! Widgets that display data.
//!
//! - [`List`](crate::widgets::List): Selectable list of items with vertical
//!   scrolling.
//! - [`Table`](crate::widgets::Table): Selectable table of rows and multiple
//!   columns with vertical scrolling.
//! - [`Scrollbar`](crate::widgets::Scrollbar): Displays the scrolling
//!   progress.

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

pub use bg_grad::BgGrad;
pub use block::Block;
pub use button::Button;
pub use grad::Grad;
pub use grid::Grid;
pub use layout::*;
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

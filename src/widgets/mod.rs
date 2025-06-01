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

/// A container widget that renders a gradient background behind its child
/// widget.
pub use bg_grad::BgGrad;
/// A widget that wrap another widget and adds border and title.
pub use block::Block;
/// A widget for rendering text with a gradient foreground color.
pub use grad::Grad;
/// A layout widget that arranges children in a grid specified by rows and
/// columns.
pub use grid::Grid;
/// A container widget that arranges child widgets in a single direction,
/// flexing their sizes based on given constraints.
pub use layout::Layout;
/// A scrollable list widget with suuport for item selection and highlighting.
pub use list::List;
/// State of the [`List`] widget, including scroll offset and selected index.
pub use list::ListState;
/// A widget that stacks its children in layers, from bottom to top.
pub use overlay::Overlay;
/// A widget combining multiple widgets implementing the [`Text`] trait into
/// single widget.
pub use paragraph::Paragraph;
/// A widget visualizing progress
pub use progress_bar::ProgressBar;
/// A wrapper widget that adds scrollability to its child when content
/// overflows.
pub use scrollable::Scrollable;
/// A scrollbar widget that can be either vertical or horizontal.
pub use scrollbar::Scrollbar;
/// Represents the scroll state shared by a [`Scrollbar`] and the app itself.
pub use scrollbar::ScrollbarState;
/// A spacer widget used for layout spacing.
pub use spacer::Spacer;
/// A widget for styling text where all characters share the same style.
pub use span::Span;
/// Enables creating [`Span`] by calling one of the functions on type
/// implementing this trait.
pub use span::ToSpan;
pub use table::Row;
/// A widget that displays a table with configurable column idths, optional
/// header and scrollable row content.
pub use table::Table;
/// State of the [`Table`] widget, including scroll offset, selected index and
/// selected column.
pub use table::TableState;
/// A container for any widget implementing the [`Widget`] trait.
pub use widget::Element;
/// Trait implemented by all the widgets.
pub use widget::Widget;

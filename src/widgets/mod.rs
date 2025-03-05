//! `widgets` is collection of types that implement `Widget` trait
//!
//! Available widgets:
//! - [`Block`]: [`Layout`] widget with addition of optional border, title
//!     and styles
//! - [`Center`]: widget for centering other widget
//! - [`Grad`]: widget that draws text with gradient foreground
//! - [`Layout`]: widget for creating layouts
//! - [`List`]: widget creating list layout with scrollbar
//! - [`Paragraph`]: widget rendering continous text using widgets implementing
//!     `Text` trait
//! - [`Spacer`]: widget for creating spaces between widgets (better layouting)
//! - [`Span`]: widget for styling text

mod bg_grad;
mod block;
mod border;
mod grad;
mod grid;
mod layout;
mod list;
mod overlay;
mod paragraph;
mod scrollable;
mod scrollbar;
mod spacer;
mod span;
mod table;
mod widget;

/// [`Layout`] widget with gradient background
pub use bg_grad::BgGrad;
/// [`Layout`] widget with border around it
pub use block::Block;
/// Border sides definition
pub use border::Border;
/// Border type enum
pub use border::BorderType;
/// Text with gradient foreground
pub use grad::Grad;
/// Creates layout by specifying columns and rows
pub use grid::Grid;
/// Creates layout flexing in one direction
pub use layout::Layout;
/// List widget with scrollbar, that displays vector of strings
pub use list::List;
/// State of the [`List`] widget
pub use list::ListState;
/// Widget that stack its children on top of each other
pub use overlay::*;
/// Chaining widgets implementing [`Text`] trait
pub use paragraph::Paragraph;
/// Widget that uses scrollbar for overflown content
pub use scrollable::*;
/// Scrollbar widget
pub use scrollbar::*;
/// Spacer widget for better layouting
pub use spacer::Spacer;
/// Widget for styling text
pub use span::Span;
/// Enables better string conversion to [`Span`]
pub use span::ToSpan;
/// Table widget with scrollbar
pub use table::Table;
/// Trait for widgets to implemen
pub use widget::*;

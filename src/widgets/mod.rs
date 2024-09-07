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

/// [`Layout`] widget with gradient background
mod bg_grad;
/// [`Layout`] widget with border around it
mod block;
/// Border sides definition and border type enum
mod border;
/// Text with gradient foreground
mod grad;
/// Creates layout by specifying columns and rows
mod grid;
/// Creates layout flexing in one direction
mod layout;
/// List widget with scrollbar, that displays vector of strings and its state
mod list;
/// Chaining widgets implementing [`Text`] trait
mod paragraph;
mod scrollable;
mod scrollbar;
/// Spacer widget for better layouting
mod spacer;
/// Widget for styling text
mod span;
/// Trait for text widgets to implement
mod text;
/// Trait for widgets to implement
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
/// Chaining widgets implementing [`Text`] trait
pub use paragraph::Paragraph;
/// Widget that uses scrollbar for overflown content
pub use scrollable::*;
pub use scrollbar::*;
/// Spacer widget for better layouting
pub use spacer::Spacer;
/// Widget for styling text
pub use span::Span;
/// Enables better string conversion to [`Span`]
pub use span::StrSpanExtension;
/// Trait for text widgets to implement
pub use text::Text;
/// Trait for widgets to implement
pub use widget::*;

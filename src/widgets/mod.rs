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

/// Layout with addition of border and title
pub mod block;
/// Defines Border sides and BorderType
pub mod border;
/// Contains Grad widget
pub mod grad;
/// Contains Layout widget
pub mod layout;
/// Contains List widget
pub mod list;
/// Constains Paragraph widget
pub mod paragraph;
/// Spacer widget for better layouting
pub mod spacer;
/// Contains Span widget
pub mod span;
/// Defines Text trait
pub mod text;
/// Defines Widget trait
pub mod widget;

#[allow(unused)]
use block::Block;
#[allow(unused)]
use grad::Grad;
#[allow(unused)]
use layout::Layout;
#[allow(unused)]
use list::List;
#[allow(unused)]
use paragraph::Paragraph;
#[allow(unused)]
use spacer::Spacer;
#[allow(unused)]
use span::Span;

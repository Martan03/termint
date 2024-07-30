/// A buffer that stores the result of the widget render method
#[allow(clippy::module_inception)]
mod buffer;
/// A buffer cell
mod cell;

/// A buffer that stores the result of the widget render method
pub use buffer::Buffer;
/// A buffer cell
pub use cell::Cell;

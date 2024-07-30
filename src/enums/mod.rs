/// ANSI colors
mod color;
/// ANSI cursor manipulation
mod cursor;
/// ANSI modifiers bitflags
mod modifier;
/// Struct representing RGB color
mod rgb;
/// Indicates how text should be wrapped
mod wrap;

/// ANSI colors
pub use color::Color;
/// ANSI cursor manipulation
pub use cursor::Cursor;
/// ANSI modifiers bitflags
pub use modifier::Modifier;
/// Struct representing RGB color
pub use rgb::RGB;
/// Indicates how text should be wrapped
pub use wrap::Wrap;

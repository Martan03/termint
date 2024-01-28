//! # termint
//! Rust library for colored printing and Terminal User Interfaces
//!
//! ## Installation:
//!
//! This library is available on [crates.io](https://crates.io/crates/termint).
//! You can add it to your projects using cargo:
//! ```terminal
//! cargo add termint
//! ```
//!
//! ## Basic example:
//!
//! Printing colored text is really easy, you can do it like this:
//!
//! ```rust
//! use termint::{
//!     enums::{bg::Bg, fg::Fg, modifier::Modifier},
//!     widgets::span::StrSpanExtension,
//! };
//!
//! println!("{}", "Cyan text".fg(Fg::Cyan));
//! println!("{}", "Cyan text on white background".fg(Fg::Cyan).bg(Bg::White));
//! println!("{}", "Bold red text".fg(Fg::Red).modifier(vec![Modifier::Bold]));
//! println!("{}", "Text with RGB value".fg(Fg::RGB(0, 249, 210)));
//! ```
//! ![image](https://github.com/Martan03/termite/assets/46300167/36408874-d9d1-4430-a204-9a60d90c2e62)
//!
//! You can see all the colors and modifiers in the
//! [documentation](https://docs.rs/termint/latest/termint/).
//!
//! ## Advanced example:
//!
//! You can also create TUIs using this library. This example shows how you can
//! use Block widget and add children to it and creating Layout:
//!
//! ```rust
//! use termint::{
//!     enums::{bg::Bg, fg::Fg},
//!     geometry::{
//!         constrain::Constrain, coords::Coords, direction::Direction,
//!     },
//!     widgets::{
//!         block::Block, border::BorderType, span::StrSpanExtension,
//!         widget::Widget,
//!     },
//! };
//!
//! // Creates main block and sets its properties
//! let mut main = Block::new()
//!     .title("Termite".to_span())
//!     .direction(Direction::Horizontal)
//!     .border_type(BorderType::Double);
//!
//! /// Creates block1 and adds span as its child
//! let mut block1 = Block::new().title("Sub block".to_span());
//! let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
//! block1.add_child(Box::new(span1), Constrain::Percent(100));
//! /// Adds block1 as child of main block
//! main.add_child(Box::new(block1), Constrain::Percent(50));
//!
//! /// Create block2 and adds span as its child
//! let mut block2 = Block::new().title("Another".to_span());
//! let span2 = "This is really cool, right?".fg(Fg::Blue);
//! block2.add_child(Box::new(span2), Constrain::Percent(100));
//! /// Adds block2 as child of main block
//! main.add_child(Box::new(block2), Constrain::Percent(50));
//!
//! /// Renders the main block which renders all the children
//! main.render(&Coords::new(1, 1), &Coords::new(30, 8));
//! ```
//! ![image](https://github.com/Martan03/termite/assets/46300167/4d820421-a607-44d5-99ec-8bd31c3c2fdf)
//!
//! ## Usage:
//!
//! Code blocks above are just examples of the usage. To see more about functions,
//! Widgets and more, please visit the
//! [documentation](https://docs.rs/termint/latest/termint/).
//!
//! ## Technologies
//!
//! Obviously this library was created in Rust, but I also used library called
//! [term-size](https://docs.rs/term_size/latest/term_size/) to get terminal size.
//!
//! ## Links
//!
//! - **Author:** [Martan03](https://github.com/Martan03)
//! - **GitHub repository:** [termint](https://github.com/Martan03/termint)
//! - **Package**: [crates.io](https://crates.io/crates/termint)
//! - **Documentation**: [docs.rs](https://docs.rs/termint/latest/termint/)
//! - **Author website:** [martan03.github.io](https://martan03.github.io)

/// Contains enums for foreground, background and more
pub mod enums;
/// Contains structs for geometry, such as Coords
pub mod geometry;
/// Contains widgets (Layout, Block, Span)
pub mod widgets;

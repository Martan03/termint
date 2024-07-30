//! # termint
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/termint?logo=rust)](https://crates.io/crates/termint)
//! [![docs.rs](https://img.shields.io/docsrs/termint?logo=rust)](https://docs.rs/termint/latest/termint/)
//! [![Crates.io Total Downloads](https://img.shields.io/crates/d/termint)](https://crates.io/crates/termint)
//!
//! Rust library for colored printing and Terminal User Interfaces
//!
//! ## Table of Contents
//! - [Installation](#installation)
//! - [Examples](#examples)
//!     - [Printing colored text](#printing-colored-text)
//!     - [More complex layout](#more-complex-layout)
//! - [Usage](#usage)
//! - [Technologies](#technologies)
//! - [Links](#links)
//!
//! ## Installation:
//!
//! This library is available on [crates.io](https://crates.io/crates/termint).
//! You can add it to your projects using cargo:
//! ```terminal
//! cargo add termint
//! ```
//!
//! ## Examples
//!
//! ### Printing colored text
//!
//! Printing colored text is really easy, you can do it like this:
//!
//! ```rust
//! # use termint::{
//! #     enums::{Modifier, Color},
//! #     widgets::StrSpanExtension
//! # };
//! // Using Span widget
//! println!("{}", "Cyan text".fg(Color::Cyan));
//! println!("{}", "Cyan text on white".fg(Color::Cyan).bg(Color::White));
//! println!("{}", "Bold red text".fg(Color::Red).modifier(Modifier::BOLD));
//! println!("{}", "Text with RGB value".fg(Color::Rgb(0, 249, 210)));
//! ```
//! ![image](https://github.com/Martan03/termint/assets/46300167/c906a565-69b5-4664-9db0-ad89ff457cbb)
//!
//! You can see all the colors and modifiers in the
//! [documentation](https://docs.rs/termint/latest/termint/).
//!
//! ### More complex layout
//!
//! You can also create TUIs using this library. This example shows how you can
//! use Block widget and add children to it and creating Layout:
//!
//! ```rust
//! # use termint::{
//! #     buffer::Buffer,
//! #     enums::Color,
//! #     geometry::{Constraint, Rect},
//! #     widgets::{Border, BorderType, Block, StrSpanExtension, Widget}
//! # };
//! // Creates main block and sets its properties
//! let mut main = Block::horizontal()
//!     .title("Termint")
//!     .border_type(BorderType::Double);
//!
//! // Creates block1 and adds span as its child
//! let mut block1 = Block::vertical().title("Sub block");
//! let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
//! block1.add_child(span1, Constraint::Percent(100));
//! // Adds block1 as child of main block
//! main.add_child(block1, Constraint::Min(0));
//!
//! // Creates block2 and adds span as its child
//! let mut block2 = Block::vertical().title("Another");
//! let span2 = "This is really cool, right?".fg(Color::Blue);
//! block2.add_child(span2, Constraint::Percent(100));
//! // Adds block2 as child of main block
//! main.add_child(block2, Constraint::Fill);
//!
//! // Renders the main block which renders all the children using Buffer
//! let mut buffer = Buffer::empty(Rect::new(1, 1, 30, 8));
//! main.render(&mut buffer);
//! buffer.render();
//! ```
//! ![image](https://github.com/Martan03/termint/assets/46300167/cdd0850b-1952-4c4b-8dec-b49c30d59f6d)
//!
//! ## Usage
//!
//! Code blocks above are just examples of the usage. To see more about functions,
//! Widgets and more, please visit the
//! [documentation](https://docs.rs/termint/latest/termint/).
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

pub mod buffer;
/// Contains enums for foreground, background and more
pub mod enums;
/// Contains structs for geometry, such as Coords
pub mod geometry;
/// Contains useful macros
pub mod macros;
pub mod style;
/// Contains Term struct
pub mod term;
/// Contains widgets (Layout, Block, Span)
pub mod widgets;

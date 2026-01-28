//! # termint
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/termint?logo=rust)](https://crates.io/crates/termint)
//! [![docs.rs](https://img.shields.io/docsrs/termint?logo=rust)](https://docs.rs/termint/latest/termint/)
//! [![Crates.io Total Downloads](https://img.shields.io/crates/d/termint)](https://crates.io/crates/termint)
//!
//! Rust library for colored printing and Terminal User Interfaces
//!
//! ## Table of Contents
//! - [How to get it](#how-to-get-it)
//!     - [With cargo](#with-cargo)
//!     - [In Cargo.toml](#in-cargotoml)
//!     - [Features](#features)
//! - [Examples](#examples)
//!     - [Printing colored text](#printing-colored-text)
//!     - [Terminal User Interface (TUI)](#terminal-user-interface-tui)
//! - [Usage](#usage)
//! - [Links](#links)
//!
//! ## How to get it
//!
//! This crate is available on [crates.io](https://crates.io/crates/termint).
//!
//! ### With cargo
//! ```terminal
//! cargo add termint
//! ```
//!
//! ### In Cargo.toml
//! ```toml
//! [dependencies]
//! termint = "0.6.0"
//! ```
//!
//! ### Features
//!
//! - `serde`: Enables serialization and deserialization of some structs.
//! - `all`: Enables all features.
//!
//! ## Examples
//!
//! ### Printing colored text
//!
//! Printing colored text is really easy, you can do it by using any `Text`
//! widget. Here is an example of using `Span` widget:
//!
//! ```rust
//! # use termint::{
//! #     enums::{Modifier, Color},
//! #     widgets::ToSpan
//! # };
//! println!("{}", "Cyan text".fg(Color::Cyan));
//! println!("{}", "Cyan text on white".fg(Color::Cyan).bg(Color::White));
//! println!("{}", "Bold red text".fg(Color::Red).modifier(Modifier::BOLD));
//! println!("{}", "Text with RGB value".fg(Color::Rgb(0, 249, 210)));
//! ```
//! ![image](https://github.com/Martan03/termint/assets/46300167/c906a565-69b5-4664-9db0-ad89ff457cbb)
//!
//! You can also use re-exported `termal` crate to print colored text:
//! ```rust
//! # use termint::termal::printcln;
//! printcln!("{'yellow italic}Yellow Italic text{'reset}");
//! printcln!("{'y i}{}{'_}", "Yellow Italic text");
//! printcln!("{'#dd0 i}{}{'_}", "Custom Yellow Italic text");
//! ```
//!
//! ### Terminal User Interface (TUI)
//!
//! The main purpose of this crate is to create Terminal User Interfaces (TUIs).
//! Example below shows minimal example of creating a TUI using `Block` widget
//! and rendering it using `Term`. You can find more examples in the `examples`
//! directory of this repository.
//!
//! ```rust
//! # use termint::{
//! #     term::Term,
//! #     enums::{Color, Border, BorderType},
//! #     geometry::{Constraint, Rect},
//! #     widgets::{Block, ToSpan, Widget}
//! # };
//! # fn example() -> Result<(), termint::Error> {
//! // Creates main block and sets its properties
//! let mut main = Block::horizontal()
//!     .title("Termint")
//!     .border_type(BorderType::Double);
//!
//! // Creates block1 and adds span as its child
//! let mut block1 = Block::vertical().title("Sub block");
//! let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
//! block1.push(span1, Constraint::Percent(100));
//! // Adds block1 as child of main block
//! main.push(block1, Constraint::Min(0));
//!
//! // Creates block2 and adds span as its child
//! let mut block2 = Block::vertical().title("Another");
//! let span2 = "This is really cool, right?".fg(Color::Blue);
//! block2.push(span2, Constraint::Percent(100));
//! // Adds block2 as child of main block
//! main.push(block2, Constraint::Fill(1));
//!
//! // Renders the main block which renders all the children using Buffer
//! let mut term = Term::new();
//! term.render(main)?;
//! # Ok(())
//! # }
//! ```
//! ![image](https://github.com/Martan03/termint/assets/46300167/cdd0850b-1952-4c4b-8dec-b49c30d59f6d)
//!
//! ### TUI examples
//!
//! ![image](https://github.com/user-attachments/assets/1e81fad9-dc56-4715-b49b-fbe9153f1b42)
//!
//! ![image](https://github.com/user-attachments/assets/660a3794-723a-494f-b28b-83377d5ebe49)
//!
//! ![image](https://github.com/user-attachments/assets/5c239669-1182-4962-8449-b76107fd574f)
//!
//! ## Usage
//!
//! Code blocks above are just examples of the usage. To see more about functions,
//! Widgets and more, please visit the
//! [documentation](https://docs.rs/termint/latest/termint/).
//!
//! You can also check the `examples` directory of this repository for more
//! examples of how to use this crate for creating TUIs.
//!
//! ## Projects
//!
//! Here is a list of some projects using termint:
//!
//! - [2048](https://github.com/Martan03/2048)
//! - [futoshiki](https://github.com/Martan03/futoshiki)
//! - [loopover](https://github.com/Martan03/loopover)
//! - [minesweeper](https://github.com/Martan03/minesweeper)
//! - [rsTimer](https://github.com/Martan03/rsTimer)
//! - [tictactoe](https://github.com/Martan03/tictactoe)
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
mod error;
/// Contains structs for geometry, such as Coords
pub mod geometry;
/// Contains useful macros
pub mod macros;
pub mod style;
/// Contains Term struct
pub mod term;
pub mod text;
/// Contains widgets (Layout, Block, Span)
pub mod widgets;

pub use error::Error;
pub use termal;

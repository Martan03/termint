//! A collection of text related types.
//!
//! This module contains a definition for [`Text`] trait, which should be
//! implemented by all the widgets styling or formatting a text.
//!
//! It also contains [`TextParser`], which is used by built-in text widgets
//! (`Span` and `Grad`). It parses the content of the text widget so it can
//! be rendered more easily.

mod text_trait;
/// A trait implemented by all the widgets that render styled or formatted
/// text.
pub use text_trait::Text;

mod parser;
/// Parses the text so it can be rendered more easily. It can be used to get
/// next line (or word) from the text using either word wrap or letter wrap.
pub use parser::TextParser;

mod text_token;
/// Text token used by the `TextParser`
pub use text_token::TextToken;

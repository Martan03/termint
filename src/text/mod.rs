mod text_trait;
pub use text_trait::Text;

mod parser;
pub(crate) use parser::TextParser;

mod text_token;
pub(crate) use text_token::TextToken;

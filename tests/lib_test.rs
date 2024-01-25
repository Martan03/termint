extern crate termite;

#[cfg(test)]
mod tests {
    use termite::{enums::{bg::Bg, fg::Fg}, span::Span};

    #[test]
    fn span_test() {
        let mut span = Span::new("Testing".to_owned());
        assert_eq!(span.get(), "\x1b[39m\x1b[49mTesting\x1b[0m");

        span.fg(Fg::Red).bg(Bg::Cyan);
        assert_eq!(span.get(), "\x1b[31m\x1b[46mTesting\x1b[0m");
    }
}
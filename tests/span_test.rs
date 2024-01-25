extern crate termite;

// To display stdout start tests with `cargo test -- --nocapture`

#[cfg(test)]
mod tests {
    use termite::{enums::{bg::Bg, fg::Fg}, span::Span};

    #[test]
    fn span_new() {
        let span = Span::new("New span");
        println!("{}", span);

        assert_eq!(span.get(), "\x1b[39m\x1b[49mNew span\x1b[0m");
    }

    #[test]
    fn span_set_fg() {
        let span = Span::new("Span fg").fg(Fg::Red);
        println!("{}", span);

        assert_eq!(span.get(), "\x1b[31m\x1b[49mSpan fg\x1b[0m");
    }

    #[test]
    fn span_set_bg() {
        let span = Span::new("Span bg").bg(Bg::White);
        println!("{}", span);

        assert_eq!(span.get(), "\x1b[39m\x1b[47mSpan bg\x1b[0m");
    }

    #[test]
    fn span_set_fg_bg() {
        let span = Span::new("Span fg bg").fg(Fg::Blue).bg(Bg::Yellow);
        println!("{}", span);

        assert_eq!(span.get(), "\x1b[34m\x1b[43mSpan fg bg\x1b[0m");
    }
}
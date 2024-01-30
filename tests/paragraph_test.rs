extern crate termint;

// These test print out the result
// To display stdout start tests with `cargo test -- --nocapture`

#[cfg(test)]
mod tests {
    use termint::{
        enums::{bg::Bg, fg::Fg, modifier::Modifier},
        widgets::{paragraph::Paragraph, span::StrSpanExtension},
    };

    /// Tests creating new paragraph
    #[test]
    fn paragraph_new() {
        let p = Paragraph::new(vec![
            "Test".fg(Fg::Blue),
            "nice".modifier(vec![Modifier::Italic]),
        ]);
        assert_eq!(
            p.get(),
            "\x1b[94m\x1b[49mTest\x1b[0m \x1b[39m\x1b[49m\x1b[3mnice\x1b[0m"
        );

        let mut p = Paragraph::new(vec![]);
        assert_eq!(p.get(), "");

        p.add("Test".fg(Fg::Black).bg(Bg::White));
        assert_eq!(p.get(), "\x1b[30m\x1b[107mTest\x1b[0m");
    }
}

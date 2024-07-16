extern crate termint;

// These test print out the result
// To display stdout start tests with `cargo test -- --nocapture`

#[cfg(test)]
mod tests {
    use termint::{
        enums::{modifier::Modifier, Color},
        mods,
        widgets::{
            span::{Span, StrSpanExtension},
            text::Text,
        },
    };

    /// Tests creating new span
    #[test]
    fn span_new() {
        // Creates span using new
        let span = Span::new("New span");
        assert_eq!(span.get(), "New span\x1b[0m");

        // Creates span from &str
        let span = "New span".to_span();
        assert_eq!(span.get(), "New span\x1b[0m");
    }

    /// Tests creating span with fg
    #[test]
    fn span_set_fg() {
        // Creates span using new
        let span = Span::new("Span fg").fg(Color::Red);
        assert_eq!(span.get(), "\x1b[91mSpan fg\x1b[0m");

        // Creates span from &str
        let mut span = "Span fg".fg(Color::Red);
        assert_eq!(span.get(), "\x1b[91mSpan fg\x1b[0m");

        // Tests modifying fg
        span = span.fg(Color::Gray);
        assert_eq!(span.get(), "\x1b[90mSpan fg\x1b[0m");

        // Tests fg with RGB value
        span = span.fg(Color::Rgb(50, 100, 150));
        assert_eq!(
            span.get(),
            format!("\x1b[38;2;{};{};{}mSpan fg\x1b[0m", 50, 100, 150)
        );
    }

    /// Tests creating span with bg
    #[test]
    fn span_set_bg() {
        // Creates span using new
        let span = Span::new("Span bg").bg(Color::White);
        assert_eq!(span.get(), "\x1b[107mSpan bg\x1b[0m");

        // Creates span from &str
        let mut span = "Span bg".bg(Color::White);
        assert_eq!(span.get(), "\x1b[107mSpan bg\x1b[0m");

        // Tests modifying bg
        span = span.bg(Color::DarkBlue);
        assert_eq!(span.get(), "\x1b[44mSpan bg\x1b[0m");

        // Tests fg with RGB value
        span = span.bg(Color::Rgb(50, 100, 150));
        assert_eq!(
            span.get(),
            format!("\x1b[48;2;{};{};{}mSpan bg\x1b[0m", 50, 100, 150)
        );
    }

    /// Tests creating span with modifiers
    #[test]
    fn span_modifier() {
        // Creates span using new
        let span = Span::new("Span modifier").modifiers(vec![
            Modifier::Bold,
            Modifier::Blink,
            Modifier::Italic,
            Modifier::Inverse,
        ]);
        assert_eq!(
            span.get(),
            "\x1b[1m\x1b[5m\x1b[3m\x1b[7mSpan modifier\x1b[0m"
        );

        // Creates span from &str
        let span = "Span modifier".modifiers(vec![
            Modifier::Bold,
            Modifier::Blink,
            Modifier::Italic,
            Modifier::Inverse,
        ]);
        assert_eq!(
            span.get(),
            "\x1b[1m\x1b[5m\x1b[3m\x1b[7mSpan modifier\x1b[0m"
        );

        // Using modifiers macro
        let span =
            "Span modifier".modifiers(mods!(Bold, Blink, Italic, Inverse));
        assert_eq!(
            span.get(),
            "\x1b[1m\x1b[5m\x1b[3m\x1b[7mSpan modifier\x1b[0m"
        );
    }

    /// Tests setting both fg and bg with RGB values
    #[test]
    fn span_set_fg_bg() {
        // Creates span using new
        let span = Span::new("Span fg bg")
            .fg(Color::Rgb(0, 150, 150))
            .bg(Color::Rgb(255, 255, 0));
        assert_eq!(
            span.get(),
            "\x1b[38;2;0;150;150m\x1b[48;2;255;255;0mSpan fg bg\x1b[0m"
        );

        // Creates span from &str
        let span = "Span fg bg"
            .fg(Color::Rgb(0, 150, 150))
            .bg(Color::Rgb(255, 255, 0));
        assert_eq!(
            span.get(),
            "\x1b[38;2;0;150;150m\x1b[48;2;255;255;0mSpan fg bg\x1b[0m"
        );
    }
}

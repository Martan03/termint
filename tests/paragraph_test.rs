extern crate termint;

// These test print out the result
// To display stdout start tests with `cargo test -- --nocapture`

#[cfg(test)]
mod tests {
    // use termint::{
    //     enums::{modifier::Modifier, Color},
    //     widgets::{paragraph::Paragraph, span::ToSpan},
    // };

    /// Tests creating new paragraph
    #[test]
    fn paragraph_new() {
        // let p = Paragraph::new(vec![
        //     Box::new("Test".fg(Color::Blue)),
        //     Box::new("nice".modifiers(vec![Modifier::Italic])),
        // ]);
        // assert_eq!(p.get(), "\x1b[94mTest\x1b[0m \x1b[39m\x1b[3mnice\x1b[0m");

        // let mut p = Paragraph::new(vec![]);
        // assert_eq!(p.get(), "");

        // p.add("Test".fg(Color::Black).bg(Color::White));
        // assert_eq!(p.get(), "\x1b[30m\x1b[107mTest\x1b[0m");
    }
}

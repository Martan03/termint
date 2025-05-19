use crate::enums::Wrap;

use super::text_token::TextToken;

/// Parses the text so it can be rendered more easily. It can be used to get
/// next line (or word, but it's mainly for line) from the text using either
/// word wrap or letter wrap.
///
/// # Examples
/// Parsing text with word wrap:
/// ```rust
/// # use termint::text::TextParser;
/// # fn get_text() -> String { String::new() }
/// let text = get_text();
/// let mut text_iter = text.chars();
///
/// // Word wrap is set by default
/// let mut parser = TextParser::new(&mut text_iter);
///
/// // Reads lines with maximum length 20 until end of the text
/// while let Some((line, len)) = parser.next_line(20) {
///     println!("{} ({} chars)", line, len);
/// }
/// ```
///
/// You can also parse the text with letter wrap, which is mainly used for
/// non-textual content, such as game board and similar:
/// ```rust
/// # use termint::{text::TextParser, enums::Wrap};
/// # fn get_text() -> String { String::new() }
/// let text = get_text();
/// let mut text_iter = text.chars();
///
/// let mut parser = TextParser::new(&mut text_iter).wrap(Wrap::Letter);
///
/// // Reads lines with maximum length 20 until end of the text
/// while let Some((line, len)) = parser.next_line(20) {
///     println!("{} ({} chars)", line, len);
/// }
/// ```
pub struct TextParser<'a> {
    text: &'a mut dyn Iterator<Item = char>,
    wrap: Wrap,
    cur: Option<char>,
    last: Option<TextToken>,
}

impl<'a> TextParser<'a> {
    /// Creates new text parser with given text.
    ///
    /// # Example
    /// ```rust
    /// # use termint::text::TextParser;
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let parser = TextParser::new(&mut text_iter);
    /// ```
    pub fn new(text: &'a mut dyn Iterator<Item = char>) -> Self {
        let cur = text.next();
        let last = match cur {
            Some(_) => None,
            None => Some(TextToken::End),
        };
        Self {
            text,
            cur,
            wrap: Wrap::default(),
            last,
        }
    }

    /// Sets wrap mode of the parser.
    ///
    /// Default value is [`Wrap::Word`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{text::TextParser, enums::Wrap};
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let parser = TextParser::new(&mut text_iter).wrap(Wrap::Letter);
    /// ```
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Gets next line from the text.
    ///
    /// Returns None when end of the text is reached, otherwise returns line
    /// and its length.
    ///
    /// # Example
    /// ```rust
    /// # use termint::{text::TextParser, enums::Wrap};
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let mut parser = TextParser::new(&mut text_iter).wrap(Wrap::Letter);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.ww_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn next_line(&mut self, max_len: usize) -> Option<(String, usize)> {
        match self.wrap {
            Wrap::Letter => self.lw_next_line(max_len),
            Wrap::Word => self.ww_next_line(max_len),
        }
    }

    /// Gets next line from the text using word wrap. Same as calling
    /// `next_line` with `wrap` set to `Wrap::Word`.
    ///
    /// Returns None when end of the text is reached, otherwise returns line
    /// and its length.
    ///
    /// # Example
    /// ```rust
    /// # use termint::text::TextParser;
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let mut parser = TextParser::new(&mut text_iter);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.ww_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn ww_next_line(&mut self, max_len: usize) -> Option<(String, usize)> {
        let (mut words, mut line_len) = match &self.last {
            Some(TextToken::Text { text, len }) => (vec![text.clone()], *len),
            _ => (vec![], 0),
        };
        // TODO: handle when word cannot fit
        self.last = None;

        loop {
            match self.next_word() {
                TextToken::Text { text, len } => {
                    let space = (line_len != 0) as usize;
                    if line_len + len + space > max_len {
                        self.last = Some(TextToken::text(text, len));
                        break;
                    }

                    words.push(text);
                    line_len += len + space;
                }
                TextToken::Newline => break,
                token => {
                    self.last = Some(token);
                    break;
                }
            }
        }

        (line_len != 0 || !self.is_end())
            .then_some((words.join(" "), line_len))
    }

    /// Gets next line from the text using letter wrap. Same as calling
    /// `next_line` with `wrap` set to `Wrap::Letter`.
    ///
    /// Returns None when end of the text is reached, otherwise returns line
    /// and its length.
    ///
    /// # Example
    /// ```rust
    /// # use termint::text::TextParser;
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let mut parser = TextParser::new(&mut text_iter);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.lw_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn lw_next_line(&mut self, max_len: usize) -> Option<(String, usize)> {
        let mut line = String::new();
        let mut line_len = 0;

        self.last = None;
        while let Some(c) = self.cur {
            if line_len >= max_len {
                return Some((line, line_len));
            }

            self.cur = self.text.next();
            if c == '\n' {
                return Some((line, line_len));
            }

            line.push(c);
            line_len += 1;
        }
        self.last = Some(TextToken::End);
        (line_len != 0).then_some((line, line_len))
    }

    /// Gets next word from the text, skips leading whitespaces.
    ///
    /// # Example
    /// ```rust
    /// # use termint::text::{TextParser, TextToken};
    /// let mut text_iter = "This is a test of termint text parser".chars();
    /// let mut parser = TextParser::new(&mut text_iter);
    ///
    /// // Gets next word from text
    /// match parser.next_word() {
    ///     TextToken::Text { text, len } => {
    ///         println!("{} ({} chars)", text, len)
    ///     },
    ///     TextToken::Newline => println!("Newline"),
    ///     TextToken::End => println!("End of text"),
    /// }
    /// ```
    pub fn next_word(&mut self) -> TextToken {
        if !self.skip_whitespace() {
            self.cur = self.text.next();
            return TextToken::Newline;
        }

        let mut word = String::new();
        let mut word_len = 0;
        while let Some(c) = self.cur {
            if c.is_whitespace() {
                break;
            }

            word.push(c);
            word_len += 1;
            self.cur = self.text.next();
        }

        match word_len {
            0 => TextToken::End,
            _ => TextToken::text(word, word_len),
        }
    }

    /// Checks if text was read to the end.
    pub fn is_end(&self) -> bool {
        self.cur.is_none() && matches!(self.last, Some(TextToken::End))
    }

    /// Skips whitespace characters except newline.
    ///
    /// Returns true when no newline, else false.
    fn skip_whitespace(&mut self) -> bool {
        while let Some(c) = self.cur {
            if c == '\n' {
                return false;
            }

            if !c.is_whitespace() {
                break;
            }
            self.cur = self.text.next();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{enums::Wrap, text::TextToken};

    use super::TextParser;

    #[test]
    fn new() {
        let mut input = "test".chars();
        let parser = TextParser::new(&mut input);

        assert_eq!(parser.cur, Some('t'));
        assert_eq!(parser.last, None);
        assert_eq!(parser.wrap, Wrap::default());
    }

    #[test]
    fn wrap() {
        let mut input = "test".chars();
        let parser = TextParser::new(&mut input).wrap(Wrap::Letter);

        assert_eq!(parser.wrap, Wrap::Letter);
    }

    #[test]
    fn is_end() {
        let mut text = "end test  ".chars();
        let mut parser = TextParser::new(&mut text);

        assert_eq!(parser.next_line(5), Some(("end".into(), 3)));
        assert_eq!(parser.last, Some(TextToken::text("test".into(), 4)));
        assert!(!parser.is_end());

        assert_eq!(parser.next_line(5), Some(("test".into(), 4)));
        assert_eq!(parser.last, Some(TextToken::End));
        assert!(parser.is_end());
    }

    #[test]
    fn is_end_empty() {
        let mut input = "".chars();
        let parser = TextParser::new(&mut input);

        assert!(parser.is_end());
    }

    #[test]
    fn skip_whitespace() {
        let cases = vec![
            ("  test", 't'),
            ("\ttest", 't'),
            ("  \ttest", 't'),
            ("  \t  test", 't'),
        ];

        for (text, expected) in cases {
            let mut text_iter = text.chars();
            let mut parser = TextParser::new(&mut text_iter);

            assert!(parser.skip_whitespace());
            assert_eq!(parser.cur, Some(expected));
        }
    }

    #[test]
    fn skip_whitespace_empty() {
        let cases = vec!["", "  \t  "];

        for text in cases {
            let mut text_iter = text.chars();
            let mut parser = TextParser::new(&mut text_iter);

            assert!(parser.skip_whitespace());
            assert_eq!(parser.cur, None);
        }
    }

    #[test]
    fn skip_whitespace_newline() {
        let cases = vec![("\n"), ("  \t \n")];

        for text in cases {
            let mut text_iter = text.chars();
            let mut parser = TextParser::new(&mut text_iter);

            assert!(!parser.skip_whitespace());
            assert_eq!(parser.cur, Some('\n'));
        }
    }

    #[test]
    fn next_word() {
        let mut text = "  \t \ntest    next \t  ".chars();
        let mut parser = TextParser::new(&mut text);

        assert_eq!(parser.next_word(), TextToken::Newline);
        assert_eq!(parser.cur, Some('t'));

        assert_eq!(parser.next_word(), TextToken::text("test".into(), 4));
        assert_eq!(parser.cur, Some(' '));

        assert_eq!(parser.next_word(), TextToken::text("next".into(), 4));

        assert_eq!(parser.next_word(), TextToken::End);
        assert_eq!(parser.next_word(), TextToken::End);
    }

    #[test]
    fn next_line_word_wrap() {
        let mut text = "This     is   \t a test of něxt  line  ".chars();
        let mut parser = TextParser::new(&mut text);

        assert_eq!(parser.next_line(15), Some(("This is a test".into(), 14)));
        assert_eq!(parser.last, Some(TextToken::text("of".into(), 2)));

        assert_eq!(parser.next_line(5), Some(("of".into(), 2)));
        assert_eq!(parser.last, Some(TextToken::text("něxt".into(), 4)));

        assert_eq!(parser.next_line(15), Some(("něxt line".into(), 9)));
        assert_eq!(parser.last, Some(TextToken::End));

        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.last, Some(TextToken::End));
    }

    #[test]
    fn next_line_word_wrap_newline() {
        let mut text = " This   is  \n a \n  \n  test ".chars();
        let mut parser = TextParser::new(&mut text);

        assert_eq!(parser.next_line(14), Some(("This is".into(), 7)));
        assert_eq!(parser.next_line(14), Some(("a".into(), 1)));
        assert_eq!(parser.next_line(14), Some(("".into(), 0)));
        assert_eq!(parser.next_line(14), Some(("test".into(), 4)));
        assert_eq!(parser.next_line(14), None);
    }

    #[test]
    fn next_line_letter_wrap() {
        let mut text = "This  is  a test  of něxt  line".chars();
        let mut parser = TextParser::new(&mut text).wrap(Wrap::Letter);

        assert_eq!(parser.next_line(15), Some(("This  is  a tes".into(), 15)));
        assert_eq!(parser.last, None);

        assert_eq!(parser.next_line(5), Some(("t  of".into(), 5)));

        assert_eq!(parser.next_line(15), Some((" něxt  line".into(), 11)));
        assert_eq!(parser.last, Some(TextToken::End));

        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.last, Some(TextToken::End));
    }

    #[test]
    fn next_line_letter_wrap_newline() {
        let mut text = " This   is  \n a \n\n  test ".chars();
        let mut parser = TextParser::new(&mut text).wrap(Wrap::Letter);

        assert_eq!(parser.next_line(14), Some((" This   is  ".into(), 12)));
        assert_eq!(parser.next_line(14), Some((" a ".into(), 3)));
        assert_eq!(parser.next_line(14), Some(("".into(), 0)));
        assert_eq!(parser.next_line(14), Some(("  test ".into(), 7)));
        assert_eq!(parser.next_line(14), None);
    }
}

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{enums::Wrap, prelude::Vec2};

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
///
/// // Word wrap is set by default
/// let mut parser = TextParser::new(&text);
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
/// let mut parser = TextParser::new(&text).wrap(Wrap::Letter);
///
/// // Reads lines with maximum length 20 until end of the text
/// while let Some((line, len)) = parser.next_line(20) {
///     println!("{} ({} chars)", line, len);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TextParser<'a> {
    text: Option<&'a str>,
    wrap: Wrap,
}

impl<'a> TextParser<'a> {
    /// Creates new text parser with given text.
    ///
    /// # Example
    /// ```rust
    /// # use termint::text::TextParser;
    /// let text = "This is a test of termint text parser";
    /// let parser = TextParser::new(text);
    /// ```
    pub fn new(text: &'a str) -> Self {
        Self {
            text: if text.is_empty() { None } else { Some(text) },
            wrap: Wrap::default(),
        }
    }

    /// Sets wrap mode of the parser.
    ///
    /// Default value is [`Wrap::Word`].
    ///
    /// # Example
    /// ```rust
    /// # use termint::{text::TextParser, enums::Wrap};
    /// let text = "This is a test of termint text parser";
    /// let parser = TextParser::new(text).wrap(Wrap::Letter);
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
    /// let text = "This is a test of termint text parser";
    /// let mut parser = TextParser::new(text).wrap(Wrap::Letter);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.ww_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn next_line(&mut self, max_width: usize) -> Option<(&'a str, usize)> {
        self.text?;
        match self.wrap {
            Wrap::Letter => self.lw_next_line(max_width),
            Wrap::Word => self.ww_next_line(max_width),
        }
    }

    /// Gets the minimum width of the text based on the height.
    pub fn width(&mut self, size: &Vec2) -> usize {
        if size.x == 0 || size.y == 0 {
            return 0;
        }

        let width = self.text.unwrap_or_default().width();
        if width == 0 {
            return 0;
        }

        let mut low = width.div_ceil(size.y).max(1);
        let mut high = width;
        while low < high {
            let mid = low + (high - low) / 2;
            let mut parser = self.clone();
            let height = parser.inner_height(mid);

            if height <= size.y {
                high = mid;
            } else {
                low = mid + 1
            }
        }
        low
    }

    /// Gets the minimum height of the text based on the width.
    pub fn height(&mut self, size: &Vec2) -> usize {
        if size.x == 0 || size.y == 0 {
            return 0;
        }
        self.inner_height(size.x)
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
    /// let text = "This is a test of termint text parser";
    /// let mut parser = TextParser::new(text);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.ww_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn ww_next_line(
        &mut self,
        max_width: usize,
    ) -> Option<(&'a str, usize)> {
        let text = self.text.unwrap_or_default();
        let mut last = None;
        let mut was_whitespace = true;
        let mut width = 0;

        for (id, grapheme) in text.grapheme_indices(true) {
            let is_whitespace = grapheme.starts_with(char::is_whitespace);
            if is_whitespace && !was_whitespace {
                last = Some((id, width));
            }

            if grapheme == "\n" {
                let (idx, w) = last.unwrap_or({
                    if was_whitespace { (0, 0) } else { (id, width) }
                });
                let line = &text[..idx];
                self.text = Some(&text[id + 1..]);
                return Some((line, w));
            }

            let grapheme_width = grapheme.width();
            if width + grapheme_width > max_width {
                let (idx, w) = last.unwrap_or((id, width));
                let line = &text[..idx];

                let rest = text[idx..].trim_start();
                self.text = if rest.is_empty() { None } else { Some(rest) };
                return Some((line, w));
            }

            width += grapheme_width;
            was_whitespace = is_whitespace;
        }

        let (idx, w) = if was_whitespace {
            last.unwrap_or((0, 0))
        } else {
            (text.len(), width)
        };
        let line = &text[..idx];

        self.text = None;
        Some((line, w))
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
    /// let text = "This is a test of termint text parser";
    /// let mut parser = TextParser::new(text);
    ///
    /// // Gets next line from text with maximum length of 20
    /// if let Some((line, len)) = parser.lw_next_line(20) {
    ///    println!("{} ({} chars)", line, len);
    /// }
    /// ```
    pub fn lw_next_line(
        &mut self,
        max_width: usize,
    ) -> Option<(&'a str, usize)> {
        let text = self.text.unwrap_or_default();
        let mut width = 0;
        for (id, grapheme) in text.grapheme_indices(true) {
            if grapheme == "\n" {
                let line = &text[..id];
                self.text = Some(&text[id + 1..]);
                return Some((line, width));
            }

            let grapheme_width = grapheme.width();
            if width + grapheme_width > max_width {
                let line = &text[..id];
                self.text = Some(&text[id..]);
                return Some((line, width));
            }

            width += grapheme_width;
        }

        self.text = None;
        Some((text, width))
    }

    /// Checks if text was read to the end.
    pub fn is_end(&self) -> bool {
        self.text.is_none()
    }
}

impl<'a> TextParser<'a> {
    fn inner_height(&mut self, width: usize) -> usize {
        let mut height = 0;
        while self.next_line(width).is_some() {
            height += 1;
        }
        height
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::Wrap;

    use super::TextParser;

    #[test]
    fn wrap() {
        let parser = TextParser::new("test").wrap(Wrap::Letter);
        assert_eq!(parser.wrap, Wrap::Letter);
    }

    #[test]
    fn is_end() {
        let mut parser = TextParser::new("end test  ");

        assert_eq!(parser.next_line(5), Some(("end".into(), 3)));
        assert!(!parser.is_end());

        assert_eq!(parser.next_line(5), Some(("test".into(), 4)));
        assert!(parser.is_end());
    }

    #[test]
    fn is_end_empty() {
        let parser = TextParser::new("");
        assert!(parser.is_end());
    }

    #[test]
    fn next_line() {
        let text = "This     is   \t a test of něxt  line  ";
        let mut parser = TextParser::new(text);

        assert_eq!(parser.next_line(15), Some(("This     is".into(), 11)));
        assert_eq!(parser.next_line(7), Some(("a test".into(), 6)));
        assert_eq!(parser.next_line(15), Some(("of něxt  line".into(), 13)));
        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.next_line(15), None);
        assert!(parser.is_end());
    }

    #[test]
    fn next_line_newline() {
        let text = " This   is  \n a \n  \n  test ";
        let mut parser = TextParser::new(text);

        assert_eq!(parser.next_line(14), Some((" This   is".into(), 10)));
        assert_eq!(parser.next_line(14), Some((" a".into(), 2)));
        assert_eq!(parser.next_line(14), Some(("".into(), 0)));
        assert_eq!(parser.next_line(14), Some(("  test".into(), 6)));
        assert_eq!(parser.next_line(14), None);
        assert!(parser.is_end());
    }

    #[test]
    fn next_line_unicode_width() {
        let text = "a 字 🚀 b";
        let mut parser = TextParser::new(text);

        assert_eq!(parser.next_line(4), Some(("a 字", 4)));
        assert_eq!(parser.next_line(4), Some(("🚀 b", 4)));
        assert!(parser.is_end());
    }

    #[test]
    fn next_line_word_too_long() {
        let text = "Hello termint";
        let mut parser = TextParser::new(text);

        assert_eq!(parser.next_line(6), Some(("Hello", 5)));
        assert_eq!(parser.next_line(5), Some(("termi", 5)));
        assert_eq!(parser.next_line(5), Some(("nt", 2)));
    }

    #[test]
    fn next_line_trailing_newline() {
        let text = "Line\n";
        let mut parser = TextParser::new(text);

        assert_eq!(parser.next_line(10), Some(("Line", 4)));
        assert_eq!(parser.next_line(10), Some(("", 0)));
        assert_eq!(parser.next_line(10), None);
    }

    #[test]
    fn next_line_letter_wrap() {
        let text = "This  is  a test  of něxt  line";
        let mut parser = TextParser::new(text).wrap(Wrap::Letter);

        assert_eq!(parser.next_line(15), Some(("This  is  a tes".into(), 15)));
        assert_eq!(parser.next_line(5), Some(("t  of".into(), 5)));
        assert_eq!(parser.next_line(15), Some((" něxt  line".into(), 11)));
        assert_eq!(parser.next_line(15), None);
        assert_eq!(parser.next_line(15), None);
        assert!(parser.is_end());
    }

    #[test]
    fn next_line_letter_wrap_newline() {
        let text = " This   is  \n a \n\n  test ";
        let mut parser = TextParser::new(text).wrap(Wrap::Letter);

        assert_eq!(parser.next_line(14), Some((" This   is  ".into(), 12)));
        assert_eq!(parser.next_line(14), Some((" a ".into(), 3)));
        assert_eq!(parser.next_line(14), Some(("".into(), 0)));
        assert_eq!(parser.next_line(14), Some(("  test ".into(), 7)));
        assert_eq!(parser.next_line(14), None);
        assert!(parser.is_end());
    }
}

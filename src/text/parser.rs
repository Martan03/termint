use crate::enums::Wrap;

use super::text_token::TextToken;

/// Parses the text so it can be rendered more easily
pub struct TextParser<'a> {
    text: &'a mut dyn Iterator<Item = char>,
    wrap: Wrap,
    cur: Option<char>,
    last: Option<TextToken>,
}

impl<'a> TextParser<'a> {
    /// Creates new text parser with given text
    pub fn new(text: &'a mut dyn Iterator<Item = char>) -> Self {
        let cur = text.next();
        Self {
            text,
            cur,
            wrap: Wrap::Word,
            last: None,
        }
    }

    /// Sets wrap mode of the parser
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Gets next line from the text
    pub fn next_line(&mut self, max_len: usize) -> TextToken {
        match self.wrap {
            Wrap::Letter => self.lw_next_line(max_len),
            Wrap::Word => self.ww_next_line(max_len),
        }
    }

    /// Reads next word in the text, skips leading whitespaces
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

    /// Checks if text was read to the end
    pub fn is_end(&self) -> bool {
        matches!(self.last, Some(TextToken::End))
    }

    /// Gets next line from the text when word wrap
    pub fn ww_next_line(&mut self, max_len: usize) -> TextToken {
        let (mut words, mut line_len) = match &self.last {
            Some(TextToken::Text { text, len }) => (vec![text.clone()], *len),
            Some(_) => return self.last.take().unwrap(),
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
                TextToken::Newline => {
                    if line_len == 0 {
                        return TextToken::Newline;
                    }
                    return TextToken::text(words.join(" "), line_len);
                }
                token => {
                    self.last = Some(token);
                    break;
                }
            }
        }

        match line_len {
            0 if self.last.is_none() => TextToken::End,
            _ => TextToken::text(words.join(" "), line_len),
        }
    }

    /// Gets next line from the text when letter wrap
    pub fn lw_next_line(&mut self, max_len: usize) -> TextToken {
        let mut line = String::new();
        let mut line_len = 0;

        self.last = None;
        while let Some(c) = self.cur {
            if line_len >= max_len {
                return TextToken::text(line, line_len);
            }

            self.cur = self.text.next();
            if c == '\n' {
                if line_len == 0 {
                    return TextToken::Newline;
                }
                return TextToken::text(line, line_len);
            }

            line.push(c);
            line_len += 1;
        }
        self.last = Some(TextToken::End);
        if line_len == 0 {
            return TextToken::End;
        }
        TextToken::text(line, line_len)
    }

    /// Skips whitespace characters except newline.
    /// Returns true when no newline, else false
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
    use crate::text::TextToken;

    use super::TextParser;

    #[test]
    fn test_next_word() {
        let text = String::from("This    is  \n   a test");
        let mut text_iter = text.chars();
        let mut parser = TextParser::new(&mut text_iter);

        assert_eq!(parser.next_word(), TextToken::text("This".into(), 4));
        assert_eq!(parser.next_word(), TextToken::text("is".into(), 2));
        assert_eq!(parser.next_word(), TextToken::Newline);
        assert_eq!(parser.next_word(), TextToken::text("a".into(), 1));
        assert_eq!(parser.next_word(), TextToken::text("test".into(), 4));
        assert_eq!(parser.next_word(), TextToken::End);
    }

    #[test]
    fn test_next_line() {
        let text = String::from("This     is   \t a test of next  line  ");
        let mut text_iter = text.chars();
        let mut parser = TextParser::new(&mut text_iter);

        assert_eq!(
            parser.next_line(14),
            TextToken::text("This is a test".into(), 14)
        );
        assert_eq!(
            parser.next_line(14),
            TextToken::text("of next line".into(), 12)
        );
        assert_eq!(parser.next_line(14), TextToken::End);
    }

    #[test]
    fn test_next_line_newline() {
        let text = String::from(" This   is  \n a \n  \n  test ");
        let mut text_iter = text.chars();
        let mut parser = TextParser::new(&mut text_iter);

        assert_eq!(parser.next_line(14), TextToken::text("This is".into(), 7));
        assert_eq!(parser.next_line(14), TextToken::text("a".into(), 1));
        assert_eq!(parser.next_line(14), TextToken::Newline);
        assert_eq!(parser.next_line(14), TextToken::text("test".into(), 4));
        assert_eq!(parser.next_line(14), TextToken::End);
    }
}

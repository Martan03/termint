use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    prelude::{Rect, TextAlign},
    text::{StrStyle, StyledStr},
};

#[derive(Debug)]
pub struct Line<'a> {
    pub parts: Vec<StyledStr<'a>>,
    pub width: usize,
}

impl<'a> Line<'a> {
    /// Creates empty [`Line`].
    pub fn empty() -> Self {
        Self {
            parts: vec![],
            width: 0,
        }
    }

    /// Pushes the given text into the [`Line`].
    ///
    /// The `text` is any type convertible into [`Cow<'a, str>`] and `style`
    /// into [`StrStyle`].
    ///
    /// The `width` is the actual character length of the `text`.
    pub fn push<T, S>(&mut self, text: T, width: usize, style: S)
    where
        T: Into<Cow<'a, str>>,
        S: Into<StrStyle>,
    {
        let frag = StyledStr::styled(text, width, style);
        self.parts.push(frag);
        self.width += width;
    }

    /// Pops the last text fragment from the current [`Line`].
    pub fn pop(&mut self) {
        if let Some(frag) = self.parts.pop() {
            self.width -= frag.width;
        }
    }

    /// Renders the current [`Line`] into the [`Buffer`].
    pub fn render(&self, buffer: &mut Buffer, rect: Rect, align: TextAlign) {
        let x_offset = match align {
            TextAlign::Left => 0,
            TextAlign::Center => rect.width().saturating_sub(self.width) / 2,
            TextAlign::Right => rect.width().saturating_sub(self.width),
        };

        let mut pos = *rect.pos();
        pos.x += x_offset;
        for frag in self.parts.iter() {
            frag.render(buffer, &pos, &rect);
            pos.x += frag.width;
        }
    }

    /// Adds given ellipsis after the text.
    ///
    /// If not enough space, it removes characters from the back until the
    /// ellipsis fit or the last character is not a whitespace.
    pub fn add_ellipsis(&mut self, max_width: usize, ellipsis: &str) {
        let width = ellipsis.width();
        let target = max_width.saturating_sub(width);

        while let Some(mut frag) = self.parts.pop() {
            self.width -= frag.width;
            let mut fwidth = frag.width;
            let mut sid = frag.text.len();

            for (idx, grapheme) in frag.text.grapheme_indices(true).rev() {
                if self.width + fwidth <= target
                    && !grapheme.starts_with(char::is_whitespace)
                {
                    break;
                }
                fwidth -= grapheme.width();
                sid = idx;
            }

            if sid > 0 {
                let trunc = format!("{}{}", &frag.text[..sid], ellipsis);
                frag.text = Cow::Owned(trunc);
                frag.width = fwidth + width;

                self.width += frag.width;
                self.parts.push(frag);
                break;
            }
        }
    }
}

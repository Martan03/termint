use std::borrow::Cow;

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
}

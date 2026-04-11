use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    prelude::Vec2,
    style::Style,
};

#[derive(Debug, Clone)]
pub enum StrStyle {
    Static(Style),
    Grad(RGB, RGB),
}

#[derive(Debug, Clone)]
pub struct StyledStr<'a> {
    pub text: Cow<'a, str>,
    pub width: usize,
    pub style: StrStyle,
}

impl<'a> StyledStr<'a> {
    /// Creates new unstyled [`StyledStr`] fragment.
    pub fn new<T>(text: T, width: usize) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            text: text.into(),
            width,
            style: StrStyle::default(),
        }
    }

    /// Creates new [`StyledStr`] fragment.
    pub fn styled<T, S>(text: T, width: usize, style: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        S: Into<StrStyle>,
    {
        Self {
            text: text.into(),
            width,
            style: style.into(),
        }
    }

    /// Sets the style into a static style.
    ///
    /// The `style` is any type convertible into [`Style`].
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = StrStyle::Static(style.into());
        self
    }

    /// Sets the style into a gradient style.
    ///
    /// The `start` and `end` are types convertible into [`RGB`].
    pub fn gradient<S, E>(mut self, start: S, end: E) -> Self
    where
        S: Into<RGB>,
        E: Into<RGB>,
    {
        self.style = StrStyle::Grad(start.into(), end.into());
        self
    }

    /// Renders the current [`StyledStr`] into the [`Buffer`].
    pub fn render(&self, buffer: &mut Buffer, pos: &Vec2) {
        match &self.style {
            StrStyle::Static(style) => {
                buffer.set_str_styled(&self.text, pos, *style)
            }
            StrStyle::Grad(start, end) => {
                self.render_grad(buffer, *pos, start, end)
            }
        }
    }

    fn render_grad(
        &self,
        buffer: &mut Buffer,
        mut pos: Vec2,
        start: &RGB,
        end: &RGB,
    ) {
        let width = self.width.saturating_sub(1) as f32;

        let (mut r, mut g, mut b) =
            (start.r as f32, start.g as f32, start.b as f32);
        let (rs, gs, bs) = get_step(start, end, width);

        for grapheme in self.text.graphemes(true) {
            let gw = grapheme.width();
            if gw == 0 {
                continue;
            }

            let color = Color::Rgb(r as u8, g as u8, b as u8);
            let style = Style::new().fg(color);
            buffer.set_str_styled(grapheme, &pos, style);

            let ssize = gw as f32;
            (r, g, b) = (r + rs * ssize, g + gs * ssize, b + bs * ssize);
            pos.x += gw;
        }
    }
}

impl Default for StrStyle {
    fn default() -> Self {
        StrStyle::Static(Style::default())
    }
}

impl<S> From<S> for StrStyle
where
    S: Into<Style>,
{
    fn from(value: S) -> Self {
        Self::Static(value.into())
    }
}

impl From<(RGB, RGB)> for StrStyle {
    fn from((start, end): (RGB, RGB)) -> Self {
        Self::Grad(start, end)
    }
}

impl<'a, T> From<(T, usize)> for StyledStr<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from((text, width): (T, usize)) -> Self {
        Self::new(text, width)
    }
}

fn get_step(start: &RGB, end: &RGB, width: f32) -> (f32, f32, f32) {
    if width <= 0. {
        return (0., 0., 0.);
    }
    (
        (end.r as f32 - start.r as f32) / width,
        (end.g as f32 - start.g as f32) / width,
        (end.b as f32 - start.b as f32) / width,
    )
}

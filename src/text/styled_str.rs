use std::{borrow::Cow, fmt::Display};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::{
    buffer::Buffer,
    enums::{Color, RGB},
    prelude::{Rect, Vec2},
    style::Style,
};

#[derive(Debug, Clone)]
pub enum StrStyle {
    Static(Style),
    LocalGrad(RGB, RGB),
    GlobalGrad(RGB, RGB),
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
    pub fn gradient<S, E>(mut self, start: S, end: E, local: bool) -> Self
    where
        S: Into<RGB>,
        E: Into<RGB>,
    {
        self.style = if local {
            StrStyle::LocalGrad(start.into(), end.into())
        } else {
            StrStyle::GlobalGrad(start.into(), end.into())
        };
        self
    }

    /// Renders the current [`StyledStr`] into the [`Buffer`].
    pub fn render(&self, buffer: &mut Buffer, pos: &Vec2, rect: &Rect) {
        let set_str = |t: &str, p: &Vec2, s: Style| {
            buffer.set_str_styled(t, p, s);
            Ok(())
        };
        _ = self.inner_render(set_str, pos, rect);
    }

    fn inner_render<F>(
        &self,
        mut set_str: F,
        pos: &Vec2,
        rect: &Rect,
    ) -> std::fmt::Result
    where
        F: FnMut(&str, &Vec2, Style) -> std::fmt::Result,
    {
        match &self.style {
            StrStyle::Static(style) => set_str(&self.text, pos, *style),
            StrStyle::LocalGrad(start, end) => {
                self.local_grad(set_str, *pos, start, end)
            }
            StrStyle::GlobalGrad(start, end) => {
                self.global_grad(set_str, rect, *pos, start, end)
            }
        }
    }

    fn local_grad<F>(
        &self,
        set_str: F,
        pos: Vec2,
        start: &RGB,
        end: &RGB,
    ) -> std::fmt::Result
    where
        F: FnMut(&str, &Vec2, Style) -> std::fmt::Result,
    {
        let (color, step) = get_step(start, end, self.width);
        self.render_grad(set_str, pos, color, step)
    }

    fn global_grad<F>(
        &self,
        set_str: F,
        area: &Rect,
        pos: Vec2,
        start: &RGB,
        end: &RGB,
    ) -> std::fmt::Result
    where
        F: FnMut(&str, &Vec2, Style) -> std::fmt::Result,
    {
        let ((r, g, b), (rs, gs, bs)) = get_step(start, end, area.width());
        let ox = (pos.x - area.x()) as f32;
        let color = (r + rs * ox, g + gs * ox, b + bs * ox);

        self.render_grad(set_str, pos, color, (rs, gs, bs))
    }

    fn render_grad<F>(
        &self,
        mut set_str: F,
        mut pos: Vec2,
        (mut r, mut g, mut b): (f32, f32, f32),
        (rs, gs, bs): (f32, f32, f32),
    ) -> std::fmt::Result
    where
        F: FnMut(&str, &Vec2, Style) -> std::fmt::Result,
    {
        for grapheme in self.text.graphemes(true) {
            let gw = grapheme.width();
            if gw == 0 {
                continue;
            }

            let color = Color::Rgb(r as u8, g as u8, b as u8);
            let style = Style::new().fg(color);
            set_str(grapheme, &pos, style)?;

            let ssize = gw as f32;
            (r, g, b) = (r + rs * ssize, g + gs * ssize, b + bs * ssize);
            pos.x += gw;
        }
        Ok(())
    }
}

impl<'a> Display for StyledStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        let set_str = |t: &str, _: &Vec2, s: Style| {
            if first {
                first = false;
                write!(f, "{}{}", s, t)
            } else {
                let fg = s.fg.map_or_else(|| "".to_string(), |fg| fg.to_fg());
                write!(f, "{}{}", fg, t)
            }
        };

        let rect = Rect::new(0, 0, self.width, 1);
        self.inner_render(set_str, &Vec2::new(0, 0), &rect)?;
        write!(f, "\x1b[0m")
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

impl<'a, T> From<(T, usize)> for StyledStr<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from((text, width): (T, usize)) -> Self {
        Self::new(text, width)
    }
}

pub(crate) fn get_step(
    start: &RGB,
    end: &RGB,
    width: usize,
) -> ((f32, f32, f32), (f32, f32, f32)) {
    let width = width.saturating_sub(1) as f32;
    let (r, g, b) = (start.r as f32, start.g as f32, start.b as f32);
    if width <= 0. {
        return ((r, g, b), (0., 0., 0.));
    }
    (
        (r, g, b),
        (
            (end.r as f32 - r) / width,
            (end.g as f32 - g) / width,
            (end.b as f32 - b) / width,
        ),
    )
}

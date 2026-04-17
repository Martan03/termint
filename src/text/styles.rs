use crate::{
    enums::{Color, Modifier, RGB},
    style::Style,
};

#[derive(Debug, Clone)]
pub enum StrStyle {
    Static(Style),
    LocalGrad(GradStyle),
    GlobalGrad(GradStyle),
}

#[derive(Debug, Clone)]
pub struct GradStyle {
    pub start: RGB,
    pub end: RGB,
    pub bg: Option<Color>,
    pub modifier: Modifier,
}

impl GradStyle {
    /// Creates new [`GradStyle`] with given start and end colors.
    ///
    /// The `start` and `end` are both types convertible into [`RGB`].
    #[must_use]
    pub fn new(start: impl Into<RGB>, end: impl Into<RGB>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            bg: None,
            modifier: Modifier::NONE,
        }
    }

    /// Sets the background color of the [`GradStyle`].
    ///
    /// The `bg` is any type convertible into `Option<Color>`.
    #[must_use]
    pub fn bg(mut self, bg: impl Into<Option<Color>>) -> Self {
        self.bg = bg.into();
        self
    }

    /// Sets modifier to the given flag
    #[must_use]
    pub fn modifier(mut self, flag: Modifier) -> Self {
        self.modifier = Modifier::empty();
        self.modifier.insert(flag);
        self
    }

    /// Adds given modifier to the already set modifiers
    #[must_use]
    pub fn add_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.insert(flag);
        self
    }

    /// Removes given modifier from the already set modifiers
    #[must_use]
    pub fn remove_modifier(mut self, flag: Modifier) -> Self {
        self.modifier.remove(flag);
        self
    }
}

impl<S, E> From<(S, E)> for GradStyle
where
    S: Into<RGB>,
    E: Into<RGB>,
{
    fn from((start, end): (S, E)) -> Self {
        Self::new(start, end)
    }
}

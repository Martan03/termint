use std::borrow::Cow;

use crate::{
    enums::{Modifier, Wrap},
    prelude::TextAlign,
    style::Style,
    widgets::Span,
};

/// Enables creating [`Span`] by calling one of the functions on type
/// implementing this trait.
///
/// It's recommended to use `std::fmt::Display` trait. Types implementing this
/// trait will contain `ToSpan` as well and can be converted to `Span`.
pub trait ToSpan {
    /// Creates [`Span`] from string and sets its style to given value
    fn style<T>(self, style: T) -> Span
    where
        T: Into<Style>;

    /// Creates [`Span`] from string and sets its modifier to given value
    fn modifier(self, modifier: Modifier) -> Span;

    /// Creates [`Span`] from string and add given modifier to it
    fn add_modifier(self, flag: Modifier) -> Span;

    /// Creates [`Span`] from string and sets its alignment to given value
    fn align(self, align: TextAlign) -> Span;

    /// Creates [`Span`] from string and sets its wrapping to given value
    fn wrap(self, wrap: Wrap) -> Span;

    /// Creates [`Span`] from string and sets its ellipsis to given value
    fn ellipsis<T>(self, ellipsis: T) -> Span
    where
        T: AsRef<str>;

    /// Converts type to [`Span`]
    fn to_span(self) -> Span;
}

macro_rules! impl_to_span {
    ($($t:ty),* $(,)?) => {
        $(
            impl ToSpan for $t {
                fn style<S>(self, style: S) -> Span
                where
                    S: Into<Style>,
                {
                    Span::new(self).style(style)
                }

                fn modifier(self, modifier: Modifier) -> Span {
                    Span::new(self).modifier(modifier)
                }

                fn add_modifier(self, flag: Modifier) -> Span {
                    Span::new(self).add_modifier(flag)
                }

                fn align(self, align: TextAlign) -> Span {
                    Span::new(self).align(align)
                }

                fn wrap(self, wrap: Wrap) -> Span {
                    Span::new(self).wrap(wrap)
                }

                fn ellipsis<R>(self, ellipsis: R) -> Span
                where
                    R: AsRef<str>,
                {
                    Span::new(self).ellipsis(ellipsis)
                }

                fn to_span(self) -> Span {
                    Span::new(self)
                }
            }
        )*
    };
}

impl_to_span!(&str, String, Cow<'_, str>);

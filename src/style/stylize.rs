use std::borrow::Cow;

use crate::style::Styleable;

macro_rules! generate_stylize_trait {
    (
        $($cname:ident => $color:ident),* $(,)?
        ;
        $($mname:ident => $modifier:ident),* $(,)?
    ) => {
        /// Trait enabling for better style applying to widgets and texts.
        ///
        /// This trait allows using builder methods for more natural styling,
        /// such as `.red()`, `.on_blue()` and `.bold()`.
        ///
        /// [`Stylize`] is automatically implemented for types implementing
        /// [`Styleable`] trait.
        ///
        /// # Example
        ///
        /// ```rust
        /// use termint::prelude::*;
        ///
        /// // Using the styles shorthands
        /// let warning = "Important message".red().on_black().bold();
        ///
        /// // You can still set color via setter
        /// let dyn_color = Color::Green;
        /// let success = "Success".fg(dyn_color).italic();
        /// ```
        pub trait Stylize: Sized {
            /// The type returned by the styling methods.
            ///
            /// Widgets usually have this as `Self`, but it allows string
            /// to implement this trait with output as `Span`, for example.
            type Output;

            /// Sets the foreground color explicitly.
            ///
            /// The `fg` can be any type convertible into `Option<Color>`. If
            /// `None` is supplied, it keeps the original foreground color.
            fn fg<T>(self, fg: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>;

            /// Sets the background color explicitly.
            ///
            /// The `bg` can be any type convertible into `Option<Color>`. If
            /// `None` is supplied, the background is transparent.
            fn bg<T>(self, bg: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>;

            /// Adds a specific modifier to the existing set of modifiers.
            fn add_modifier(
                self,
                modifier: crate::enums::Modifier
            ) -> Self::Output;

            /// Removes a specific modifier if it is currently set.
            fn remove_modifier(
                self,
                modifier: crate::enums::Modifier
            ) -> Self::Output;

            $(
                #[doc = concat!(
                    "Sets the foreground color to [`Color::",
                    stringify!($color),
                    "`](crate::enums::Color::",
                    stringify!($color), ")."
                )]
                fn $cname(self) -> Self::Output {
                    self.fg(crate::enums::Color::$color)
                }

                #[doc = concat!(
                    "Sets the background color to [`Color::",
                    stringify!($color),
                    "`](crate::enums::Color::",
                    stringify!($color), ")."
                )]
                paste::paste! {
                    fn [<on_ $cname>](self) -> Self::Output {
                        self.bg(crate::enums::Color::$color)
                    }
                }
            )*

            $(
                #[doc = concat!(
                    "Applies the [`Modifier::",
                    stringify!($modifier),
                    "`](crate::enums::Modifier::",
                    stringify!($modifier),
                    ") modifier."
                )]
                fn $mname(self) -> Self::Output {
                    self.add_modifier(crate::enums::Modifier::$modifier)
                }
            )*
        }

        impl<W> Stylize for W
        where
            W: crate::style::Styleable,
        {
            type Output = W;

            fn fg<T>(mut self, color: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>
            {
                self.style_mut().fg = color.into();
                self
            }

            fn bg<T>(mut self, color: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>
            {
                self.style_mut().bg = color.into();
                self
            }

            fn add_modifier(
                mut self,
                modifier: crate::enums::Modifier
            ) -> Self::Output
            {
                self.style_mut().modifier.insert(modifier);
                self
            }

            fn remove_modifier(
                mut self,
                modifier: crate::enums::Modifier
            ) -> Self::Output
            {
                self.style_mut().modifier.remove(modifier);
                self
            }
        }
    };
}

macro_rules! impl_stylize {
    (
        $($from:ty => $to:ty),* $(,)?
    ) => {
        $(
            impl Stylize for $from {
                type Output = $to;

                fn fg<T>(self, color: T) -> Self::Output
                where
                    T: Into<Option<crate::enums::Color>>
                {
                    let mut widget: $to = self.into();
                    widget.style_mut().fg = color.into();
                    widget
                }

                fn bg<T>(self, color: T) -> Self::Output
                where
                    T: Into<Option<crate::enums::Color>>
                {
                    let mut widget: $to = self.into();
                    widget.style_mut().bg = color.into();
                    widget
                }

                fn add_modifier(
                    self,
                    modifier: crate::enums::Modifier
                ) -> Self::Output
                {
                    let mut widget: $to = self.into();
                    widget.style_mut().modifier.insert(modifier);
                    widget
                }

                fn remove_modifier(
                    self,
                    modifier: crate::enums::Modifier
                ) -> Self::Output
                {
                    let mut widget: $to = self.into();
                    widget.style_mut().modifier.remove(modifier);
                    widget
                }
            }
        )*
    };
}

generate_stylize_trait! {
    black => Black,
    dark_red => DarkRed,
    dark_green => DarkGreen,
    dark_yellow => DarkYellow,
    dark_blue => DarkBlue,
    dark_magenta => DarkMagenta,
    dark_cyan => DarkCyan,
    light_gray => LightGray,
    gray => Gray,
    red => Red,
    green => Green,
    yellow => Yellow,
    blue => Blue,
    magenta => Magenta,
    cyan => Cyan,
    white => White,
    ;
    bold => BOLD,
    dim => DIM,
    italic => ITALIC,
    underline => UNDERLINED,
    blink => BLINK,
    inverse => INVERSED,
    hidden => HIDDEN,
    strike => STRIKED,
}

impl_stylize! {
    &str => crate::widgets::Span,
    String => crate::widgets::Span,
    Cow<'_, str> => crate::widgets::Span,
}

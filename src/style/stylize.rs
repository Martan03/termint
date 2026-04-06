use crate::style::Styleable;

macro_rules! generate_stylize_trait {
    (
        $($cname:ident => $color:ident),* $(,)?
        ;
        $($mname:ident => $modifier:ident),* $(,)?
    ) => {
        pub trait Stylize: Sized {
            type Output;

            fn fg<T>(self, color: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>;

            fn bg<T>(self, color: T) -> Self::Output
            where
                T: Into<Option<crate::enums::Color>>;

            fn add_modifier(
                self,
                modifier: crate::enums::Modifier
            ) -> Self::Output;

            fn remove_modifier(
                self,
                modifier: crate::enums::Modifier
            ) -> Self::Output;

            $(
                fn $cname(self) -> Self::Output {
                    self.fg(crate::enums::Color::$color)
                }

                paste::paste! {
                    fn [<on_ $cname>](self) -> Self::Output {
                        self.bg(crate::enums::Color::$color)
                    }
                }
            )*

            $(
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
                    widget.style_mut().fg = color.into();
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
    default => Default,
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
}

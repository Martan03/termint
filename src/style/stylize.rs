macro_rules! generate_stylize_trait {
    (
        $($cname:ident => $color:ident),* $(,)?
        ;
        $($mname:ident => $modifier:ident),* $(,)?
    ) => {
        pub trait Stylize {
            type Output;

            $(
                fn $cname(self) -> Self::Output;

                paste::paste! {
                    fn [<on_ $cname>](self) -> Self::Output;
                }
            )*

            $(
                fn $mname(self) -> Self::Output;
            )*
        }

        impl<W> Stylize for W
        where
            W: crate::style::Styleable,
        {
            type Output = W;

            $(
                fn $cname(mut self) -> Self::Output {
                    self.style_mut().fg = Some(crate::enums::Color::$color);
                    self
                }

                paste::paste! {
                    fn [<on_ $cname>](mut self) -> Self::Output {
                        self.style_mut().bg =
                            Some(crate::enums::Color::$color);
                        self
                    }
                }
            )*

            $(
                fn $mname(mut self) -> Self::Output {
                    self.style_mut().modifier.insert(
                        crate::enums::Modifier::$modifier
                    );
                    self
                }
            )*
        }
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

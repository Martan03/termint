use crate::enums::rgb::RGB;

/// ANSI colors
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    /// Black (fg: 30, bg: 40)
    Black,
    /// Red (fg: 31, bg: 41)
    DarkRed,
    /// Green (fg: 32, bg: 42)
    DarkGreen,
    /// Yellow (fg: 33, bg: 43)
    DarkYellow,
    /// Blue (fg: 34, bg: 44)
    DarkBlue,
    /// Magenta (fg: 35, bg: 45)
    DarkMagenta,
    /// Cyan (fg: 36, bg: 46)
    DarkCyan,
    /// White (fg: 37, bg: 47)
    LightGray,
    /// Bright Black (fg: 90, bg: 100)
    Gray,
    /// Bright Red (fg: 91, bg: 101)
    Red,
    /// Bright Green (fg: 92, bg: 102)
    Green,
    /// Bright Yellow (fg: 93, bg: 103)
    Yellow,
    /// Bright Blue (fg: 94, bg: 104)
    Blue,
    /// Bright Magenta (fg: 95, bg: 105)
    Magenta,
    /// Bright Cyan (fg: 96, bg: 106)
    Cyan,
    /// Bright White (fg: 97, bg: 107)
    White,
    /// 8-bit 256 color
    Indexed(u8),
    /// RGB color
    Rgb(u8, u8, u8),
    /// HSL color
    Hsl(f64, f64, f64),
    /// Hex color
    Hex(u32),
    /// Resets the foreground and background color
    #[default]
    Default,
}

impl Color {
    /// Converts [`Color`] to corresponding foreground ANSI color
    pub fn to_fg(&self) -> String {
        match self {
            Color::Black => "\x1b[30m".to_string(),
            Color::DarkRed => "\x1b[31m".to_string(),
            Color::DarkGreen => "\x1b[32m".to_string(),
            Color::DarkYellow => "\x1b[33m".to_string(),
            Color::DarkBlue => "\x1b[34m".to_string(),
            Color::DarkMagenta => "\x1b[35m".to_string(),
            Color::DarkCyan => "\x1b[36m".to_string(),
            Color::LightGray => "\x1b[37m".to_string(),
            Color::Gray => "\x1b[90m".to_string(),
            Color::Red => "\x1b[91m".to_string(),
            Color::Green => "\x1b[92m".to_string(),
            Color::Yellow => "\x1b[93m".to_string(),
            Color::Blue => "\x1b[94m".to_string(),
            Color::Magenta => "\x1b[95m".to_string(),
            Color::Cyan => "\x1b[96m".to_string(),
            Color::White => "\x1b[97m".to_string(),
            Color::Indexed(i) => format!("\x1b[38;5;{i}m"),
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{r};{g};{b}m"),
            Color::Hsl(h, s, l) => {
                let rgb = RGB::from_hsl(*h, *s, *l);
                format!("\x1b[38;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
            }
            Color::Hex(val) => {
                let rgb = RGB::from_hex(*val);
                format!("\x1b[38;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
            }
            Color::Default => "\x1b[39m".to_string(),
        }
    }

    /// Converts [`Color`] to corresponding background ANSI color
    pub fn to_bg(&self) -> String {
        match self {
            Color::Black => "\x1b[40m".to_string(),
            Color::DarkRed => "\x1b[41m".to_string(),
            Color::DarkGreen => "\x1b[42m".to_string(),
            Color::DarkYellow => "\x1b[43m".to_string(),
            Color::DarkBlue => "\x1b[44m".to_string(),
            Color::DarkMagenta => "\x1b[45m".to_string(),
            Color::DarkCyan => "\x1b[46m".to_string(),
            Color::LightGray => "\x1b[47m".to_string(),
            Color::Gray => "\x1b[100m".to_string(),
            Color::Red => "\x1b[101m".to_string(),
            Color::Green => "\x1b[102m".to_string(),
            Color::Yellow => "\x1b[103m".to_string(),
            Color::Blue => "\x1b[104m".to_string(),
            Color::Magenta => "\x1b[105m".to_string(),
            Color::Cyan => "\x1b[106m".to_string(),
            Color::White => "\x1b[107m".to_string(),
            Color::Indexed(i) => format!("\x1b[48;5;{i}m"),
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{r};{g};{b}m"),
            Color::Hsl(h, s, l) => {
                let rgb = RGB::from_hsl(*h, *s, *l);
                format!("\x1b[48;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
            }
            Color::Hex(val) => {
                let rgb = RGB::from_hex(*val);
                format!("\x1b[48;2;{};{};{}m", rgb.r, rgb.g, rgb.b)
            }
            Color::Default => "\x1b[49m".to_string(),
        }
    }

    fn str_to_hex(value: &str) -> Option<u32> {
        let value = value.trim_start_matches('#');
        let Ok(radix) = u32::from_str_radix(value, 16) else {
            return None;
        };

        let hex = match value.len() {
            1 => {
                let val = radix | (radix << 4);
                (val << 16) | (val << 8) | val
            }
            2 => (radix << 16) | (radix << 8) | radix,
            3 => {
                let r = (radix & 0xf00) << 12 | (radix & 0xf00) << 8;
                let g = (radix & 0x0f0) << 8 | (radix & 0x0f0) << 4;
                let b = (radix & 0x00f) << 4 | (radix & 0x00f);
                r | g | b
            }
            6 => radix,
            _ => return None,
        };
        Some(hex)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::Hex(value)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::Rgb(r, g, b)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((h, s, l): (f64, f64, f64)) -> Self {
        Self::Hsl(h, s, l)
    }
}

impl From<&str> for Color {
    /// Converts given string to [`Color`].
    ///
    /// # Panics
    /// Panics if the string is unknown color
    fn from(value: &str) -> Self {
        match value {
            "black" | "bl" => Self::Black,
            "dark_red" | "dr" => Self::DarkRed,
            "dark_green" | "dg" => Self::DarkGreen,
            "dark_yellow" | "dy" => Self::DarkYellow,
            "dark_blue" | "db" => Self::DarkBlue,
            "dark_magenta" | "dm" => Self::DarkMagenta,
            "dark_cyan" | "dc" => Self::DarkCyan,
            "light_gray" | "light_grey" | "lg" => Self::LightGray,
            "gray" | "grey" | "gr" => Self::Gray,
            "red" | "r" => Self::Red,
            "green" | "g" => Self::Green,
            "yellow" | "y" => Self::Yellow,
            "blue" | "b" => Self::Blue,
            "magenta" | "m" => Self::Magenta,
            "cyan" | "c" => Self::Cyan,
            "white" | "w" => Self::White,
            "default" | "d" => Self::Default,
            hex if hex.starts_with('#') => {
                let Some(hex) = Self::str_to_hex(hex) else {
                    panic!("invalid hex color provided");
                };
                Self::Hex(hex)
            }
            _ => panic!("unknown color"),
        }
    }
}

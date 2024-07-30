use std::fmt;

/// Enum for modifier ANSI codes
///
/// You can use macro to get vector with Modifers:
/// ```rust
/// # use termint::{enums::modifier::Modifier, mods};
/// // Gets vector with Bold and Italic modifier
/// let modifiers = mods!(Bold, Italic);
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Modifier(u8);

impl Modifier {
    /// Bold mode
    pub const BOLD: u8 = 0b0000_0001;
    // Dim/faint mode
    pub const DIM: u8 = 0b0000_0010;
    // Italic mode
    pub const ITALIC: u8 = 0b0000_0100;
    // Underline mode
    pub const UNDERLINED: u8 = 0b0000_1000;
    // Blinking mode
    pub const BLINK: u8 = 0b0001_0000;
    // Inverse/reverse mode
    pub const INVERSED: u8 = 0b0010_0000;
    // Hidden/invisible mode
    pub const HIDDEN: u8 = 0b0100_0000;
    // Strikethrough mode
    pub const STRIKED: u8 = 0b1000_0000;

    /// Gets empty modifier
    pub fn empty() -> Self {
        Self(0)
    }

    /// Clears all the modifiers
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// Gets the value of the [`Modifier`]
    pub fn val(&self) -> u8 {
        self.0
    }

    /// Adds given flag to the [`Modifier`]
    pub fn add(&mut self, flag: u8) {
        self.0 |= flag;
    }

    /// Subs given flag from the [`Modifier`]
    pub fn sub(&mut self, flag: u8) {
        self.0 &= !flag;
    }

    pub fn to_ansi(&self) -> &'static str {
        ""
    }
}

impl fmt::Display for Modifier {
    /// Automatically converts [`Modifier`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut codes = Vec::new();

        if self.0 & Self::BOLD != 0 {
            codes.push("1");
        }
        if self.0 & Self::DIM != 0 {
            codes.push("2");
        }
        if self.0 & Self::ITALIC != 0 {
            codes.push("3");
        }
        if self.0 & Self::UNDERLINED != 0 {
            codes.push("4");
        }
        if self.0 & Self::BLINK != 0 {
            codes.push("5");
        }
        if self.0 & Self::INVERSED != 0 {
            codes.push("7");
        }
        if self.0 & Self::HIDDEN != 0 {
            codes.push("8");
        }
        if self.0 & Self::STRIKED != 0 {
            codes.push("9");
        }

        if codes.is_empty() {
            Ok(())
        } else {
            write!(f, "\x1b[1;34;{}m", codes.join(";"))
        }
    }
}

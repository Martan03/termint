use std::fmt;

use bitflags::bitflags;

bitflags! {
    /// Modifier struct used for bitflags for the modifiers
    ///
    /// Since modifier is bitflag, you can combine multiple modifiers using `|`, or
    /// you can use `add` method, or use `modifiers!` macro.
    ///
    /// ```rust
    /// # use termint::{enums::Modifier, modifiers};
    /// // Combines using binary or
    /// let modifiers = Modifier::BOLD | Modifier::ITALIC;
    ///
    /// // Combines using the Modifier struct
    /// let mut modifiers: Modifier = Modifier::empty();
    /// modifiers.insert(Modifier::BOLD);
    /// modifiers.insert(Modifier::ITALIC);
    ///
    /// // Uses macro (does the same as binary or in shorter way)
    /// let modifiers = modifiers!(BOLD, ITALIC);
    /// ```
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Modifier: u8 {
        /// Bold mode
        const BOLD = 0b0000_0001;
        // Dim/faint mode
        const DIM = 0b0000_0010;
        // Italic mode
        const ITALIC = 0b0000_0100;
        // Underline mode
        const UNDERLINED = 0b0000_1000;
        // Blinking mode
        const BLINK = 0b0001_0000;
        // Inverse/reverse mode
        const INVERSED = 0b0010_0000;
        // Hidden/invisible mode
        const HIDDEN = 0b0100_0000;
        // Strikethrough mode
        const STRIKED = 0b1000_0000;

        const NONE = 0;
    }
}

impl fmt::Display for Modifier {
    /// Automatically converts [`Modifier`] to ANSI code when printing
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut codes = Vec::new();

        if self.bits() & Modifier::BOLD.bits() != 0 {
            codes.push("1");
        }
        if self.bits() & Self::DIM.bits() != 0 {
            codes.push("2");
        }
        if self.bits() & Self::ITALIC.bits() != 0 {
            codes.push("3");
        }
        if self.bits() & Self::UNDERLINED.bits() != 0 {
            codes.push("4");
        }
        if self.bits() & Self::BLINK.bits() != 0 {
            codes.push("5");
        }
        if self.bits() & Self::INVERSED.bits() != 0 {
            codes.push("7");
        }
        if self.bits() & Self::HIDDEN.bits() != 0 {
            codes.push("8");
        }
        if self.bits() & Self::STRIKED.bits() != 0 {
            codes.push("9");
        }

        if codes.is_empty() {
            Ok(())
        } else {
            write!(f, "\x1b[{}m", codes.join(";"))
        }
    }
}

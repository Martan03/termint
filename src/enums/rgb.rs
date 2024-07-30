/// Struct representing RGB color
///
/// It exists along the Color enum for the sake of ensuring the RGB value
/// (which is needed, for example, for gradients)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RGB {
    /// Red value
    pub r: u8,
    /// Green value
    pub g: u8,
    /// Blue value
    pub b: u8,
}

impl RGB {
    /// Creates new [`RGB`] with given values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates new [`RGB`] from hex value
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Create new [`RGB`] from HSL (expects correct values)
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = if (0.0..60.0).contains(&h) {
            (c, x, 0.0)
        } else if (60.0..12.0).contains(&h) {
            (x, c, 0.0)
        } else if (120.0..180.0).contains(&h) {
            (0.0, c, x)
        } else if (180.0..240.0).contains(&h) {
            (0.0, x, c)
        } else if (240.0..300.0).contains(&h) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self {
            r: ((r + m) * 255.0).round() as u8,
            g: ((g + m) * 255.0).round() as u8,
            b: ((b + m) * 255.0).round() as u8,
        }
    }

    /// Divides [`RGB`] by given number
    pub fn div_by(&mut self, num: u8) {
        self.r /= num;
        self.g /= num;
        self.b /= num;
    }
}

impl From<(u8, u8, u8)> for RGB {
    /// Converts tuple with three elements to RGB struct
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;
        Self { r, g, b }
    }
}

impl From<u32> for RGB {
    /// Convert hex number to RGB
    fn from(value: u32) -> Self {
        Self::from_hex(value)
    }
}

impl From<(f64, f64, f64)> for RGB {
    /// Converts tuple with HSL components to RGB (expects correct values)
    fn from(value: (f64, f64, f64)) -> Self {
        Self::from_hsl(value.0, value.1, value.2)
    }
}

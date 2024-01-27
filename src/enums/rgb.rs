/// RGB struct containing 3 items (r, g, b)
#[derive(Debug)]
pub struct RGB {
    /// Red value
    pub r: u8,
    /// Green value
    pub g: u8,
    /// Blue value
    pub b: u8,
}

impl RGB {
    /// Creates new RGB with given values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Converts hex (in format #rrggbb) to RGB
    /// If error occures, returns None, else return Some and RGB struct
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 7 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[1..3], 16),
                u8::from_str_radix(&hex[3..5], 16),
                u8::from_str_radix(&hex[5..7], 16),
            ) {
                return Some(Self { r, g, b });
            }
            return None;
        }
        None
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

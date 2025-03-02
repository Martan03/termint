/// Text alignment options
#[derive(Default, Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Direction enum
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

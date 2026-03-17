/// Direction enum
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

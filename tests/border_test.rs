#[cfg(test)]
mod tests {
    use termite::{
        borders,
        widgets::border::{Border, BorderType},
    };

    /// Test borders macro usage for combining border sides
    #[test]
    fn border_macro_test() {
        assert_eq!(borders!(TOP, BOTTOM), Border::TOP | Border::BOTTOM);
        assert_eq!(borders!(TOP, RIGHT), Border::TOP | Border::RIGHT);
        assert_eq!(borders!(TOP, RIGHT, LEFT, BOTTOM), Border::ALL);
    }

    /// Tests border type getting right characters
    #[test]
    fn border_type_test() {
        let border_type = BorderType::Normal;
        assert_eq!(
            border_type.get(Border::TOP),
            border_type.get(Border::BOTTOM)
        );
        assert_eq!(
            border_type.get(Border::RIGHT),
            border_type.get(Border::LEFT)
        );
        assert_eq!(border_type.get(Border::TOP), '─');
        assert_eq!(border_type.get(Border::LEFT), '│');
        assert_eq!(border_type.get(borders!(TOP, LEFT)), '┌');
        assert_eq!(border_type.get(borders!(TOP, RIGHT)), '┐');
        assert_eq!(border_type.get(borders!(BOTTOM, LEFT)), '└');
        assert_eq!(border_type.get(borders!(BOTTOM, RIGHT)), '┘');
    }
}

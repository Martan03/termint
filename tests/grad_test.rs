extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{enums::Color, modifiers, text::Text, widgets::Grad};

    /// Tests creating new grad
    #[test]
    fn grad_new() {
        let grad = Grad::new("Gradient", (0, 220, 255), (175, 80, 255));
        let assert_val = format!(
            "{}G{}r{}a{}d{}i{}e{}n{}t\x1b[0m",
            Color::Rgb(0, 220, 255).to_fg(),
            Color::Rgb(25, 200, 255).to_fg(),
            Color::Rgb(50, 180, 255).to_fg(),
            Color::Rgb(75, 160, 255).to_fg(),
            Color::Rgb(100, 140, 255).to_fg(),
            Color::Rgb(125, 120, 255).to_fg(),
            Color::Rgb(150, 100, 255).to_fg(),
            Color::Rgb(175, 80, 255).to_fg(),
        );
        assert_eq!(grad.get(), assert_val);
    }

    /// Tests creating grad with white background, bold and underline
    #[test]
    fn grad_with_modifiers() {
        let grad = Grad::new("Gradient", (0, 220, 255), (175, 80, 255))
            .modifier(modifiers!(BOLD, UNDERLINED))
            .bg(Color::White);
        let assert_val = format!(
            "\x1b[1;34;1;4m{}{}G{}r{}a{}d{}i{}e{}n{}t\x1b[0m",
            Color::White.to_bg(),
            Color::Rgb(0, 220, 255).to_fg(),
            Color::Rgb(25, 200, 255).to_fg(),
            Color::Rgb(50, 180, 255).to_fg(),
            Color::Rgb(75, 160, 255).to_fg(),
            Color::Rgb(100, 140, 255).to_fg(),
            Color::Rgb(125, 120, 255).to_fg(),
            Color::Rgb(150, 100, 255).to_fg(),
            Color::Rgb(175, 80, 255).to_fg(),
        );
        assert_eq!(grad.get(), assert_val);
    }
}

extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{
        enums::{bg::Bg, fg::Fg},
        widgets::grad::Grad,
    };

    /// Creates creating new grad
    #[test]
    fn grad_new() {
        let grad = Grad::new("Gradient", (0, 220, 255), (200, 60, 255));
        let assert_val = format!(
            "{}{}G{}r{}a{}d{}i{}e{}n{}t\x1b[0m",
            Bg::Default,
            Fg::RGB(0, 220, 255),
            Fg::RGB(25, 200, 255),
            Fg::RGB(50, 180, 255),
            Fg::RGB(75, 160, 255),
            Fg::RGB(100, 140, 255),
            Fg::RGB(125, 120, 255),
            Fg::RGB(150, 100, 255),
            Fg::RGB(175, 80, 255),
        );
        assert_eq!(grad.get(), assert_val);
    }
}

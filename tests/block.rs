#[cfg(test)]
mod tests {
    use termal::formatc;
    use termint::{
        borders,
        buffer::Buffer,
        enums::{BorderType, Color, Modifier},
        geometry::Rect,
        style::Style,
        widgets::{cache::Cache, Block, Widget},
    };

    #[test]
    fn border_title_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = Block::empty().title("Test").into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 7, 4), &mut cache);

        let expected = formatc!(
            "          \n  ┌Test─┐ \n  │     │ \n  │     │ \n  └─────┘ {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn styled_border_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let style = Style::new()
            .fg(Color::Cyan)
            .bg(Color::Black)
            .modifier(Modifier::BOLD);
        let bg = Block::empty().border_style(style).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 7, 4), &mut cache);

        let expected = formatc!(
            "          \n  {style}┌─────┐\x1b[0m \n  {style}│\x1b[0m     \
            {style}│\x1b[0m \n  {style}│\x1b[0m     {style}│\x1b[0m \n  \
            {style}└─────┘\x1b[0m {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn border_type_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = Block::empty().border_type(BorderType::Double).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 7, 4), &mut cache);

        let expected = formatc!(
            "          \n  ╔═════╗ \n  ║     ║ \n  ║     ║ \n  ╚═════╝ {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn border_side_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = Block::empty().borders(borders!(TOP, LEFT)).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 7, 4), &mut cache);

        let expected = formatc!(
            "          \n  ┌────── \n  │       \n  │       \n  │       {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn content_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = Block::vertical().bg(Color::Red).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 7, 4), &mut cache);

        let expected = formatc!(
            "          \n  ┌─────┐ \n  │{}     {}│ \n  │{}     {}│ \n  \
            └─────┘ {'_}",
            Color::Red.to_bg(),
            Color::Default.to_bg(),
            Color::Red.to_bg(),
            Color::Default.to_bg(),
        );
        assert_eq!(buffer.to_string(), expected);
    }
}

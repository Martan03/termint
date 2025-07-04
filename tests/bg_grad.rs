#[cfg(test)]
mod tests {
    use termal::formatc;
    use termint::{
        buffer::Buffer,
        geometry::Rect,
        widgets::{cache::Cache, BgGrad, Widget},
    };

    #[test]
    fn horizontal_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = BgGrad::horizontal(0x0088FF, 0xFF8800).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 4, 2), &mut cache);

        let grad =
            formatc!("{'#0088FF_} {'#5588AA_} {'#AA8855_} {'#FF8800_} ");
        let expected = formatc!(
            "          \n  {grad}\x1b[49m    \n  {grad}\x1b[49m    \n          \
            \n          {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn vertical_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let bg = BgGrad::vertical(0x0088FE, 0xFE8800).into();
        cache.diff(&bg);
        bg.render(&mut buffer, Rect::new(3, 2, 4, 3), &mut cache);

        let expected = formatc!(
            "          \n  {'#0088FE_}{}\x1b[49m    \n  {'#7F887F_}{}\
            \x1b[49m    \n  {'#FE8800_}{}\x1b[49m    \n          {'_}",
            formatc!(" ").repeat(4),
            formatc!(" ").repeat(4),
            formatc!(" ").repeat(4)
        );
        assert_eq!(buffer.to_string(), expected);
    }
}

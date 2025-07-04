#[cfg(test)]
mod tests {
    use termal::formatc;
    use termint::{
        buffer::Buffer,
        enums::Color,
        geometry::{Constraint, Rect},
        widgets::{cache::Cache, Layout, Widget},
    };

    #[test]
    fn background_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let layout = Layout::vertical().bg(Color::Red).into();
        cache.diff(&layout);
        layout.render(&mut buffer, Rect::new(3, 2, 6, 3), &mut cache);

        let bg =
            format!("{}      {}", Color::Red.to_bg(), Color::Default.to_bg());
        let expected = formatc!(
            "          \n  {bg}  \n  {bg}  \n  {bg}  \n          {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn padding_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let mut layout = Layout::vertical().padding((1, 2, 2, 3));
        layout.push(Layout::vertical().bg(Color::Red), Constraint::Fill(1));
        let layout = layout.into();

        cache.diff(&layout);
        layout.render(&mut buffer, rect, &mut cache);

        let bg =
            format!("{}     {}", Color::Red.to_bg(), Color::Default.to_bg());
        let expected = formatc!(
            "          \n   {bg}  \n   {bg}  \n          \n          {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }

    #[test]
    fn center_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);
        let mut cache = Cache::new();

        let mut layout = Layout::horizontal().center();
        let mut inner = Layout::vertical().center();
        inner.push(Layout::vertical().bg(Color::Red), Constraint::Length(3));
        layout.push(inner, Constraint::Length(2));

        let layout = layout.into();
        cache.diff(&layout);
        layout.render(&mut buffer, rect, &mut cache);

        let bg = format!("{}  {}", Color::Red.to_bg(), Color::Default.to_bg());
        let expected = formatc!(
            "          \n    {bg}    \n    {bg}    \n    {bg}    \
            \n          {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }
}

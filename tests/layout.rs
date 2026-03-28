#[cfg(test)]
mod tests {
    use termal::formatc;
    use termint::{
        buffer::Buffer,
        enums::Color,
        geometry::{Constraint, Rect},
        widgets::{Element, Layout, LayoutNode, Widget},
    };

    #[test]
    fn background_render() {
        let rect = Rect::new(1, 1, 10, 5);
        let mut buffer = Buffer::empty(rect);

        let layout: Element<()> = Layout::vertical().bg(Color::Red).into();
        let mut node = LayoutNode::new(&layout);
        layout.layout(&mut node, Rect::new(3, 2, 6, 3));
        layout.render(&mut buffer, &node);

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

        let mut layout = Layout::vertical().padding((1, 2, 2, 3));
        layout.push(Layout::vertical().bg(Color::Red), Constraint::Fill(1));
        let layout: Element<()> = layout.into();

        let mut node = LayoutNode::new(&layout);
        layout.layout(&mut node, rect);
        layout.render(&mut buffer, &node);

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

        let mut layout = Layout::horizontal().center();
        let mut inner = Layout::vertical().center();
        inner.push(Layout::vertical().bg(Color::Red), Constraint::Length(3));
        layout.push(inner, Constraint::Length(2));

        let layout: Element<()> = layout.into();
        let mut node = LayoutNode::new(&layout);
        layout.layout(&mut node, rect);
        layout.render(&mut buffer, &node);

        let bg = format!("{}  {}", Color::Red.to_bg(), Color::Default.to_bg());
        let expected = formatc!(
            "          \n    {bg}    \n    {bg}    \n    {bg}    \
            \n          {'_}"
        );
        assert_eq!(buffer.to_string(), expected);
    }
}

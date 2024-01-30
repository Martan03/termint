extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{
        enums::{fg::Fg, modifier::Modifier},
        geometry::coords::Coords,
        widgets::{
            paragraph::Paragraph, span::StrSpanExtension, widget::Widget,
        },
    };

    #[allow(unused)]
    fn test() {
        // Creates new Paragraph filled with spans
        let mut p = Paragraph::new(vec![
            "This is a text in".fg(Fg::Yellow),
            "paragraph".modifier(vec![Modifier::Bold]).fg(Fg::Cyan),
            "and it adds".to_span(),
            "separator".modifier(vec![Modifier::Italic]),
        ]);
        // You can also add child later
        p.add("between each span".to_span());

        // Paragraph can be printed like this
        println!("{p}");

        // Or you can render it on given position and with given size
        p.render(&Coords::new(1, 1), &Coords::new(20, 10));
    }
}

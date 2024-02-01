use crate::{geometry::coords::Coords, widgets::widget::Widget};

#[derive(Debug)]
pub struct Term;

impl Term {
    pub fn render<T>(widget: T) -> Result<(), &'static str>
    where
        T: Widget + 'static,
    {
        if let Some((w, h)) = term_size::dimensions() {
            widget.render(&Coords::new(1, 1), &Coords::new(w, h - 1));
            Ok(())
        } else {
            Err("Cannot determine terminal size")
        }
    }
}

use crate::{
    enums::{cursor::Cursor, wrap::Wrap},
    geometry::coords::Coords,
};

use super::{span::Span, widget::Widget};

/// [`Paragraph`] allow to use multiple [`Span`] in one Widget,
/// separating them with set separator. Spans are placed after each
/// other, which you can't really achieve with Layout
pub struct Paragraph {
    children: Vec<Span>,
    separator: String,
    wrap: Wrap,
}

impl Paragraph {
    /// Creates new [`Paragraph`]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            separator: " ".to_string(),
            wrap: Wrap::Word,
        }
    }

    /// Sets [`Paragraph`] separator to given string
    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    /// Adds [`Span`] to [`Paragraph`]
    pub fn add(&mut self, span: Span) {
        self.children.push(span);
    }

    /// Gets length of the [`Paragraph`]
    pub fn len(&self) -> usize {
        let mut len = 0;
        for child in self.children.iter() {
            len += child.width(&Coords::new(0, 1));
        }
        len + (self.children.len() - 1) * self.separator.len()
    }

    /// Renders [`Paragraph`] with word wrap
    fn render_word_wrap(&self, pos: &Coords, size: &Coords) {
        let mut coords = Coords::new(0, pos.y);
        print!("{}", Cursor::Pos(pos.x, pos.y));

        for child in self.children.iter() {
            print!("{}", child.get_ansi());

            let words: Vec<&str> =
                child.get_text().split_whitespace().collect();
            for word in words {
                let mut print_str = if coords.x == 0 {
                    format!("{word}")
                } else {
                    format!(" {word}")
                };

                if coords.x + print_str.len() > size.x {
                    coords.y += 1;
                    if coords.y >= pos.y + size.y || word.len() > size.x {
                        break;
                    }

                    coords.x = 0;
                    print_str = word.to_string();
                    print!("{}", Cursor::Pos(pos.x, coords.y));
                }

                print!("{print_str}");
                coords.x += print_str.len();
            }
        }
    }

    /// Renders [`Paragraph`] with letter wrap
    fn render_letter_wrap(&self, pos: &Coords, size: &Coords) {
        let chars = size.x * size.y;

        let mut x = 0;
        let mut y = 0;
        print!("{}", Cursor::Pos(pos.x, pos.y));
        for child in self.children.iter() {
            print!("{}", child.get_ansi());
            for c in child.get_text().chars() {
                if x + y * size.x >= chars {
                    break;
                }

                if x >= size.x {
                    y += 1;
                    x = 0;
                    print!("{}", Cursor::Pos(pos.x, pos.y + y));
                }

                print!("{c}");
                x += 1;
            }

            if x + self.separator.len() <= size.x {
                print!("{}", self.separator);
                x += self.separator.len();
            }
        }
    }
}

impl Widget for Paragraph {
    fn render(
        &self,
        pos: &crate::geometry::coords::Coords,
        size: &crate::geometry::coords::Coords,
    ) {
        match self.wrap {
            Wrap::Letter => self.render_letter_wrap(pos, size),
            Wrap::Word => self.render_word_wrap(pos, size),
        }
        println!("\x1b[0m");
    }

    fn height(&self, _size: &crate::geometry::coords::Coords) -> usize {
        todo!()
    }

    fn width(&self, _size: &crate::geometry::coords::Coords) -> usize {
        todo!()
    }
}

use crate::geometry::{constrain::Constrain, coords::Coords};

use super::{layout::Layout, spacer::Spacer, widget::Widget};

pub struct Center {
    layout: Layout,
}

impl Center {
    /// Creates new [`Center`] with centering in both directions
    /// Currently work in progress
    pub fn new<T>(child: T, width: Constrain, height: Constrain) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        let mut ver = Layout::vertical();
        let ver_spacer = Center::spacer_size(&height);
        ver.add_child(Spacer::new(), ver_spacer);
        ver.add_child(child.into(), height);
        ver.add_child(Spacer::new(), ver_spacer);

        let mut layout = Layout::horizontal();
        let hor_spacer = Center::spacer_size(&width);
        layout.add_child(Spacer::new(), hor_spacer);
        layout.add_child(ver, width);
        layout.add_child(Spacer::new(), hor_spacer);

        Self { layout }
    }

    /// Creates new [`Center`] with horizontal centering
    pub fn horizontal<T>(child: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        let mut layout = Layout::horizontal();
        layout.add_child(Spacer::new(), Constrain::Fill);
        layout.add_child(child.into(), Constrain::Min(0));
        layout.add_child(Spacer::new(), Constrain::Fill);

        Self { layout }
    }

    /// Creates new [`Center`] with vertical centering
    pub fn vertical<T>(child: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        let mut layout = Layout::vertical();
        layout.add_child(Spacer::new(), Constrain::Fill);
        layout.add_child(child.into(), Constrain::Min(0));
        layout.add_child(Spacer::new(), Constrain::Fill);

        Self { layout }
    }
}

impl Widget for Center {
    fn render(&self, pos: &Coords, size: &Coords) {
        self.layout.render(pos, size);
    }

    fn height(&self, size: &Coords) -> usize {
        self.layout.height(size)
    }

    fn width(&self, size: &Coords) -> usize {
        self.layout.width(size)
    }
}

impl Center {
    fn spacer_size(size: &Constrain) -> Constrain {
        match size {
            Constrain::Fill => Constrain::Length(0),
            _ => Constrain::Fill,
        }
    }
}

impl From<Center> for Box<dyn Widget> {
    fn from(value: Center) -> Self {
        Box::new(value)
    }
}

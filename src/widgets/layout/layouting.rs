use crate::{geometry::Padding, prelude::Rect, widgets::layout::LayoutNode};

/// Handles the layout calculation of the widget with padding.
///
/// It then uses recursion to its children using the same padded rectangle.
pub fn padded<P, F>(
    node: &mut LayoutNode,
    area: Rect,
    padding: P,
    mut layout_children: F,
) where
    P: Into<Padding>,
    F: FnMut(&mut LayoutNode, Rect),
{
    layout_children(node, area.inner(padding.into()));
}

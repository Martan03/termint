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
    if !node.is_dirty && !node.has_dirty_child {
        return;
    }

    let rect = area.inner(padding.into());
    layout_children(node, rect);

    node.area = area;
    node.is_dirty = false;
    node.has_dirty_child = false;
}

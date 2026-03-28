use crate::{prelude::Rect, widgets::Widget};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutNode {
    pub area: Rect,
    pub children: Vec<LayoutNode>,
    pub is_dirty: bool,
    pub has_dirty_child: bool,
}

impl LayoutNode {
    /// Constructs new [`LayoutNode`] recursively based on the given [`Widget`]
    /// tree.
    pub fn new<M>(widget: &dyn Widget<M>) -> Self
    where
        M: Clone + 'static,
    {
        let children = widget.children();
        Self {
            area: Rect::default(),
            children: children.iter().map(|c| LayoutNode::new(*c)).collect(),
            is_dirty: true,
            has_dirty_child: true,
        }
    }

    pub fn diff<M>(&mut self, old: &dyn Widget<M>, new: &dyn Widget<M>)
    where
        M: Clone + 'static,
    {
        if old.layout_hash() != new.layout_hash() {
            self.is_dirty = true;
        }

        let old_children = old.children();
        let new_children = new.children();
        if old_children.len() != new_children.len() {
            self.children
                .resize(new_children.len(), LayoutNode::default());
            self.is_dirty = true;
        }

        for i in 0..new_children.len() {
            let Some(old_child) = old_children.get(i) else {
                self.children[i] = LayoutNode::new(new_children[i]);
                continue;
            };

            self.children[i].diff(*old_child, new_children[i]);
            if self.children[i].is_dirty || self.children[i].has_dirty_child {
                self.has_dirty_child = true;
            }
        }
    }
}

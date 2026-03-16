use crate::{prelude::Rect, widgets::Widget};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutNode {
    pub area: Rect,
    pub children: Vec<LayoutNode>,
    pub is_dirty: bool,
}

impl LayoutNode {
    pub fn diff(&mut self, old: &dyn Widget, new: &dyn Widget) {
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
            if let Some(old_child) = old_children.get(i) {
                self.children[i].diff(*old_child, new_children[i]);
                if self.children[i].is_dirty {
                    self.is_dirty = true;
                }
            } else {
                self.children[i].is_dirty = true;
                self.is_dirty = true;
            }
        }
    }
}

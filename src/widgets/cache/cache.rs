use std::any::{Any, TypeId};

use crate::widgets::{Element, Widget};

#[derive(Debug, Default)]
pub struct Cache {
    pub widget_type: Option<TypeId>,
    pub local: Option<Box<dyn Any>>,
    pub children: Vec<Cache>,
}

impl Cache {
    /// Creates new empty [`Cache`] node
    pub fn new() -> Self {
        Self {
            widget_type: None,
            local: None,
            children: vec![],
        }
    }

    /// Gets the local cache with specified type
    ///
    /// Returns None when no local cache is set or it is not of given type.
    pub fn local<T: 'static>(&mut self) -> Option<&mut T> {
        self.local.as_mut()?.downcast_mut::<T>()
    }

    /// Updates the cache tree when it differs from the widget tree.
    pub fn diff(&mut self, widget: &Element) {
        if self.widget_type != Some(widget.type_id) {
            self.widget_type = Some(widget.type_id);
            self.local = None;
            self.children.clear();
        }

        let children = widget.children();
        if self.children.len() != children.len() {
            self.children.resize_with(children.len(), Cache::new);
        }

        for (cache, child) in self.children.iter_mut().zip(children) {
            cache.diff(child);
        }
    }
}

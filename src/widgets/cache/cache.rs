use std::any::{Any, TypeId};

use crate::widgets::{Element, Widget};

/// A persistent state tree that mirrors the structure of the widget tree.
///
/// The [`Cache`] is used to store data that needs to persist across frames,
/// such as layout calculations, internal widget state, or any other expensive
/// calculations.
///
/// The [`Cache`] cache is used internally by the widgets. Only time you will
/// use the [`Cache`] directly is when implementing custom widget or custom
/// application lifecycle (not using [`Term`](crate::term::Term)).
#[derive(Debug, Default)]
pub struct Cache {
    /// The [`TypeId`] of the widget that used this node.
    pub widget_type: Option<TypeId>,
    /// The actual cached data.
    pub local: Option<Box<dyn Any>>,
    /// Recrusive cache nodes for the widget's children
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

    /// Gets a mutable reference to the local cache data if it matches type
    /// `T`.
    ///
    /// Returns `None` if the stored type doesn't match `T` or the cache wasn't
    /// initialized.
    pub fn local<T: 'static>(&mut self) -> Option<&mut T> {
        self.local.as_mut()?.downcast_mut::<T>()
    }

    /// Updates the cache tree to match the new widget tree.
    ///
    /// This compares the current cache node with the given `widget`.
    ///
    /// # Behavior
    /// 1. Type mismatch: if the `widget` has a different [`TypeId`] than what
    ///   is stored in `self.widget_type`, the local cache and children are
    ///   dropped and reset.
    /// 2. Structure change: if the `widget` has different number of children,
    ///   it is resized to match it.
    /// 3. Recursion: the method is called recursively for all children.
    pub fn diff<M: Clone + 'static>(&mut self, widget: &Element<M>) {
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

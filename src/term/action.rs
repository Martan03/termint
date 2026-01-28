use bitflags::bitflags;

bitflags! {
    /// Used by [`Application`] to signal required action to the [`Term`].
    /// This, for example, allows rendering only when needed or a clean way to
    /// quit the [`Term`] main loop in the [`Term::run`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Action: u8 {
        /// Do nothing
        const NONE = 0b0000;
        /// Re-render the previous widget tree. Used when only widget's state
        /// changed, such as selected list item.
        const RERENDER = 0b0001;
        /// Rebuild the widget tree by calling `view()` and renders it.
        const RENDER = 0b0010;
        /// Quits the main loop
        const QUIT = 0b0100;
    }
}

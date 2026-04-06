use crate::style::Style;

/// The trait for widgets to support better styling via `Stylize` trait.
///
/// By implementing `Styleable`, widget automatically gains access to the
/// [`Stylize`](crate::style::Stylize) trait, which provided builder methods
/// for more natural styling, such as `.red()`, '.on_blue()` and `.bold()`.
///
/// # Example
///
/// ```rust
/// use termint::{prelude::*, style::Styleable};
///
/// #[derive(Default)]
/// pub struct CustomWidget {
///     style: Style,
/// }
///
/// impl Styleable for CustomWidget {
///     fn style_mut(&mut self) -> &mut Style {
///         &mut self.style
///     }
/// }
///
/// let widget = CustomWidget::default().red().on_blue().bold();
/// ```
pub trait Styleable {
    /// Returns a mutable reference to the widget's internal [`Style`].
    fn style_mut(&mut self) -> &mut Style;
}

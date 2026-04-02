use std::{
    io::{Write, stdout},
    panic::{set_hook, take_hook},
    sync::Once,
    time::Instant,
};

use termal::codes::{
    DISABLE_ALTERNATIVE_BUFFER, ENABLE_ALTERNATIVE_BUFFER, ERASE_SCREEN,
    HIDE_CURSOR, SHOW_CURSOR,
};

use crate::{
    buffer::Buffer,
    error::Error,
    geometry::{Padding, Rect, Vec2},
    prelude::MouseEvent,
    term::{
        Action, Application, Frame,
        backend::{Backend, DefaultBackend, Event},
        disable_bracketed_paste, disable_mouse_capture,
        enable_bracketed_paste, enable_mouse_capture,
    },
    widgets::{Element, EventResult, LayoutNode, Spacer, Widget},
};

static HOOK_SET: Once = Once::new();

/// The main entry point for terminal management and rendering.
///
/// [`Term`] provides two ways to build the TUI:
/// 1. **Framework mode**: using [`Term::run`] with [`Application`] trait
///    (recommended).
/// 2. **Manual mode**: manually managing the application lifetime.
///
/// # Example (framework mode):
///
/// Simple app definition and usage. This assumes at least one backend feature
/// is enabled (by default crossterm backend is used).
///
/// ```rust,no_run
/// use termint::prelude::*;
///
/// struct MyApp;
///
/// impl Application for MyApp {
///     type Message = ();
///
///     fn view(&self, _frame: &Frame) -> Element<Self::Message> {
///         "Your UI here".into()
///     }
///
///     fn event(&mut self, event: Event) -> Action {
///         match event {
///             Event::Key(k) if k.code == KeyCode::Char('q') => Action::QUIT,
///             _ => Action::NONE,
///         }
///     }
/// }
///
/// fn main() -> Result<(), Error> {
///     Term::default().setup()?.run(&mut MyApp)
/// }
/// ```
///
/// # Example (manual mode):
///
/// ```rust
/// use termint::prelude::*;
///
/// # fn example() -> Result<(), termint::Error> {
/// let main = Block::vertical().title("Example".to_span());
/// // Creates new Term with padding 1 on every side
/// let mut term = Term::<(), _>::default().padding(1);
/// term.render(main)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Term<M: 'static = (), B: Backend = DefaultBackend> {
    backend: B,
    prev: Option<Buffer>,
    prev_widget: Option<Element<M>>,
    small: Option<Element<M>>,
    layout: LayoutNode,
    padding: Padding,
    setuped: bool,
    mouse_enabled: bool,
    paste_enabled: bool,
    last_size: Vec2,
}

impl<M, B: Backend> Term<M, B>
where
    M: Clone + 'static,
{
    /// Creates new [`Term`] with the specified backend
    pub fn new() -> Self {
        Self::custom(B::default())
    }

    /// Creates new [`Term`] and prepares the terminal using [`Term::setup`].
    ///
    /// The terminal is restored automatically when [`Term`] is dropped.
    pub fn init() -> Result<Self, Error> {
        let mut term = Self::new();
        term = term.setup()?;
        Ok(term)
    }

    /// Prepares the terminal: enables the alternate buffer, clears screen,
    /// hides cursor and enable raw mode.
    ///
    /// When using manual rendering ([`Term::render`] or [`Term::draw`]), you
    /// should call this once at the start of your program.
    ///
    /// The terminal is restored automatically when [`Term`] is dropped.
    pub fn setup(mut self) -> Result<Self, Error> {
        if !self.setuped {
            B::enable_raw_mode()?;
            print!(
                "{}{}{}",
                ENABLE_ALTERNATIVE_BUFFER, ERASE_SCREEN, HIDE_CURSOR
            );
            _ = stdout().flush();

            HOOK_SET.call_once(|| {
                let hook = take_hook();
                set_hook(Box::new(move |pi| {
                    Self::restore();
                    hook(pi);
                }));
            });

            self.setuped = true;
        }
        Ok(self)
    }

    /// Creates new [`Term`] with the given backend
    pub fn custom(backend: B) -> Self {
        Self {
            backend,
            prev: None,
            prev_widget: None,
            small: None,
            layout: LayoutNode::default(),
            padding: Padding::default(),
            setuped: false,
            mouse_enabled: false,
            paste_enabled: false,
            last_size: Vec2::default(),
        }
    }

    /// Enables mouse events backend capture ([`Event::Mouse`]).
    pub fn with_mouse(mut self) -> Self {
        if !self.mouse_enabled {
            enable_mouse_capture();
            self.mouse_enabled = true;
        }
        self
    }

    /// Enables bracketed paste mode, which allows capturing [`Event::Paste`].
    pub fn with_paste(mut self) -> Self {
        if !self.paste_enabled {
            enable_bracketed_paste();
            self.paste_enabled = true;
        }
        self
    }

    /// Disable mouse events backend capture.
    pub fn disable_mouse(&mut self) {
        if self.mouse_enabled {
            disable_mouse_capture();
            self.mouse_enabled = false;
        }
    }

    /// Disables bracketed paste mode.
    pub fn disable_paste(&mut self) {
        if self.paste_enabled {
            disable_bracketed_paste();
            self.paste_enabled = false;
        }
    }

    /// Sets [`Padding`] of the [`Term`] to given value.
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets small screen of the [`Term`], which is displayed if rendering
    /// cannot fit.
    pub fn small_screen<T>(mut self, small_screen: T) -> Self
    where
        T: Into<Element<M>>,
    {
        self.small = Some(small_screen.into());
        self
    }

    /// Clears the layout cache of the [`Term`].
    ///
    /// This is useful when the widget's layout changes, but the layout doesn't
    /// update. This shouldn't happen though, so use it only when really
    /// needed.
    pub fn clear_layout(&mut self) {
        self.layout = LayoutNode::default();
    }

    /// Renders given widget on full screen with set padding. Displays small
    /// screen when cannot fit (only when `small_screen` is set).
    pub fn render<T>(&mut self, widget: T) -> Result<(), Error>
    where
        T: Into<Element<M>>,
    {
        let widget = widget.into();
        let rect = self.get_rect()?;
        self.render_widget(widget, rect);
        Ok(())
    }

    /// Renders widget given by the `get_widget` function on full screen with
    /// set padding. Displays small screen when cannot fit (only when
    /// `small_screen` is set).
    ///
    /// Same as [`Term::render`], but the widget is provided by given closure,
    /// which also accepts [`Frame`], which contains context about currently
    /// rendering frame. This allows different layouts based on terminal size
    /// for example.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// # fn example() -> Result<(), termint::Error> {
    /// let main = Block::<(), _>::vertical().title("Example".to_span());
    /// // Creates new Term with padding 1 on every side
    /// let mut term = Term::<(), _>::default().padding(1);
    /// term.draw(|frame| {
    ///     if frame.area().width() < 100 {
    ///         "Width is smaller then 100.".into()
    ///     } else {
    ///         "Width is larger then 100.".into()
    ///     }
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn draw<F>(&mut self, get_widget: F) -> Result<(), Error>
    where
        F: FnOnce(&Frame) -> Element<M>,
    {
        let rect = self.get_rect()?;
        let frame = Frame::new(rect);
        let widget = get_widget(&frame);
        self.render_widget(widget, rect);
        Ok(())
    }

    /// Re-renders the last rendered widget tree.
    ///
    /// This is efficient way of updating UI, when you only update states of
    /// widgets that don't change the layout structure (such as
    /// [`List`](crate::widgets::List) selected item).
    pub fn rerender(&mut self) -> Result<(), Error> {
        let wid = self.prev_widget.take().ok_or(Error::NoPreviousWidget)?;

        let rect = self.get_rect()?;
        self.render_widget(wid, rect);
        Ok(())
    }

    /// Starts the application main loop and handles the terminal state.
    ///
    /// This method does the following:
    /// 1. Main loop: polls for events and updates the state:
    ///     - Calls [`Application::event`] on event
    ///         - Automatically renders on resize
    ///     - Calls [`Application::update`] each tick
    ///     - Runs corresponding merged action from previous calls
    /// 2. Ends the main loop when [`Action::QUIT`] is received
    ///
    /// # Example
    ///
    /// ```rust
    /// use termint::prelude::*;
    ///
    /// # #[derive(Default)]
    /// # struct MyApp;
    /// # impl Application for MyApp {
    /// #     type Message = ();
    /// #
    /// #     fn view(&self, _frame: &Frame) -> Element {
    /// #         Spacer::new().into()
    /// #     }
    /// # }
    /// # fn example() -> Result<(), termint::Error> {
    /// let mut term = Term::default();
    /// let mut app = MyApp::default();
    /// term.run(&mut app)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn run<A>(&mut self, app: &mut A) -> Result<(), Error>
    where
        A: Application<Message = M>,
    {
        self.draw(|f| app.view(f))?;

        let mut last_tick = Instant::now();
        let timeout = app.poll_timeout();
        loop {
            let mut action = Action::NONE;
            if let Some(event) = self.backend.read_event(timeout)? {
                match event {
                    Event::Mouse(ref e) => action |= self.handle_mouse(app, e),
                    Event::Resize(_, _) => action |= Action::RENDER,
                    _ => {}
                }
                action |= app.event(event);
            }

            let now = Instant::now();
            let delta = now.duration_since(last_tick);
            last_tick = now;
            action |= app.update(delta);

            if action.contains(Action::QUIT) {
                break;
            } else if action.contains(Action::RENDER) {
                self.draw(|f| app.view(f))?;
            } else if action.contains(Action::RERENDER) {
                self.rerender()?;
            }
        }

        Ok(())
    }

    /// Restores the terminal: disables the alternate buffer and shows cursor
    ///
    /// Note restore is done automatically and should be used only when you
    /// want to restore the buffer before the [`Term`] is dropped.
    pub fn restore() {
        if B::is_raw_mode_enabled() {
            print!("{}{}", DISABLE_ALTERNATIVE_BUFFER, SHOW_CURSOR);
            _ = B::disable_raw_mode();
        }
        disable_mouse_capture();
        disable_bracketed_paste();
        _ = stdout().flush();
    }

    /// Gets size of the terminal
    pub fn get_size(&self) -> Option<(usize, usize)> {
        self.backend.get_size().ok()
    }

    fn handle_mouse<A>(&mut self, app: &mut A, event: &MouseEvent) -> Action
    where
        A: Application<Message = M>,
    {
        let Some(root) = &self.prev_widget else {
            return Action::NONE;
        };
        match root.on_event(&self.layout, event) {
            EventResult::None => Action::NONE,
            EventResult::Consumed => Action::RERENDER,
            EventResult::Response(m) => app.message(m),
        }
    }

    fn render_widget(&mut self, widget: Element<M>, rect: Rect) {
        let mut buffer = Buffer::empty(rect);

        let dummy: Element<M> = Spacer::new().into();
        let prev = self.prev_widget.as_ref().unwrap_or(&dummy);
        match &self.small {
            Some(small)
                if rect.width() < widget.width(rect.size())
                    || rect.height() < widget.height(rect.size()) =>
            {
                self.layout.diff(prev, small);
                small.layout(&mut self.layout, rect);
                small.render(&mut buffer, &self.layout);
            }
            _ => {
                self.layout.diff(prev, &widget);
                widget.layout(&mut self.layout, rect);
                widget.render(&mut buffer, &self.layout);
            }
        };

        self.prev_widget = Some(widget);
        match &self.prev {
            Some(prev) => buffer.render_diff(prev),
            None => buffer.render(),
        }
        self.prev = Some(buffer);
    }

    fn get_rect(&mut self) -> Result<Rect, Error> {
        let (w, h) = self.get_size().ok_or(Error::UnknownTerminalSize)?;

        let pos = Vec2::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Vec2::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );

        if size != self.last_size {
            self.last_size = size;
        }

        Ok(Rect::from_coords(pos, size))
    }
}

impl<M> Default for Term<M, DefaultBackend> {
    fn default() -> Self {
        Self {
            backend: Default::default(),
            prev: Default::default(),
            prev_widget: Default::default(),
            small: Default::default(),
            layout: Default::default(),
            padding: Default::default(),
            setuped: false,
            mouse_enabled: false,
            paste_enabled: false,
            last_size: Default::default(),
        }
    }
}

impl<M, B: Backend> Drop for Term<M, B> {
    fn drop(&mut self) {
        if self.mouse_enabled {
            disable_mouse_capture();
        }
        if self.paste_enabled {
            disable_bracketed_paste();
        }
        if self.setuped {
            print!("{}{}", DISABLE_ALTERNATIVE_BUFFER, SHOW_CURSOR);
            _ = stdout().flush();
            _ = B::disable_raw_mode();
        }
    }
}

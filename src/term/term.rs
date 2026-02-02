use std::{
    io::{stdout, Write},
    panic::{set_hook, take_hook},
    sync::Once,
};

use termal::{
    codes::{
        DISABLE_ALTERNATIVE_BUFFER, ENABLE_ALTERNATIVE_BUFFER, ERASE_SCREEN,
        HIDE_CURSOR, SHOW_CURSOR,
    },
    raw::{disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, term_size},
};

use crate::{
    buffer::Buffer,
    error::Error,
    geometry::{Padding, Rect, Vec2},
    term::{
        backend::{Backend, DefaultBackend, Event, NoBackend},
        Action, Application, Frame,
    },
    widgets::{cache::Cache, Element, Widget},
};

static HOOK_SET: Once = Once::new();

/// The main entry point for terminal management and rendering.
///
/// [`Term`] provides two ways to build the TUI:
/// 1. **Framework mode**: using [`Term::run`] with [`Application`] trait
///     (recommended).
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
///     fn view(&self, _frame: &Frame) -> Element {
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
/// let mut term = Term::default().padding(1);
/// term.render(main)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Term<B = NoBackend> {
    backend: B,
    prev: Option<Buffer>,
    prev_widget: Option<Element>,
    small: Option<Element>,
    cache: Cache,
    padding: Padding,
    setuped: bool,
    last_size: Vec2,
}

impl<B: Default> Term<B> {
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
}

impl<B> Term<B> {
    /// Creates new [`Term`] with the given backend
    pub fn custom(backend: B) -> Self {
        Self {
            backend: backend,
            prev: None,
            prev_widget: None,
            small: None,
            cache: Cache::default(),
            padding: Padding::default(),
            setuped: false,
            last_size: Vec2::default(),
        }
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
            enable_raw_mode()?;
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

    /// Sets [`Padding`] of the [`Term`] to given value.
    pub fn padding<T: Into<Padding>>(mut self, padding: T) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets small screen of the [`Term`], which is displayed if rendering
    /// cannot fit.
    pub fn small_screen<T>(mut self, small_screen: T) -> Self
    where
        T: Into<Element>,
    {
        self.small = Some(small_screen.into());
        self
    }

    /// Renders given widget on full screen with set padding. Displays small
    /// screen when cannot fit (only when `small_screen` is set).
    pub fn render<T>(&mut self, widget: T) -> Result<(), Error>
    where
        T: Into<Element>,
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
    /// let main = Block::vertical().title("Example".to_span());
    /// // Creates new Term with padding 1 on every side
    /// let mut term = Term::default().padding(1);
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
        F: FnOnce(&Frame) -> Element,
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
    /// widgets that don't change the layout structure (such as [`List`]
    /// selected item).
    pub fn rerender(&mut self) -> Result<(), Error> {
        let wid = self.prev_widget.take().ok_or(Error::NoPreviousWidget)?;

        let rect = self.get_rect()?;
        self.render_widget(wid, rect);
        Ok(())
    }

    /// Clears the cache of the [`Term`].
    ///
    /// This is useful when a widget's state changes, but the cache doesn't
    /// automatically update. After clearing the cache, the next rendering will
    /// recalculate the sizes and positions of widgets.
    pub fn clear_cache(&mut self) {
        self.cache = Cache::default();
    }

    /// Restores the terminal: disables the alternate buffer and shows cursor
    ///
    /// Note restore is done automatically and should be used only when you
    /// want to restore the buffer before the [`Term`] is dropped.
    pub fn restore() {
        if is_raw_mode_enabled() {
            print!("{}{}", DISABLE_ALTERNATIVE_BUFFER, SHOW_CURSOR);
            _ = stdout().flush();
            _ = disable_raw_mode();
        }
    }

    /// Gets size of the terminal
    pub fn get_size() -> Option<(usize, usize)> {
        term_size().ok().map(|s| (s.char_width, s.char_height))
    }
}

impl<B: Backend> Term<B> {
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
    pub fn run<A: Application>(&mut self, app: &mut A) -> Result<(), Error> {
        self.draw(|f| app.view(f))?;

        let timeout = app.poll_timeout();
        loop {
            let mut action = Action::NONE;
            if let Some(event) = self.backend.read_event(timeout)? {
                match event {
                    Event::Resize(_, _) => action |= Action::RENDER,
                    _ => {}
                }
                action |= app.event(event);
            }

            action |= app.update();

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
}

impl<B> Term<B> {
    fn render_widget(&mut self, widget: Element, rect: Rect) {
        let mut buffer = Buffer::empty(rect);
        match &self.small {
            Some(small)
                if rect.width() < widget.width(rect.size())
                    || rect.height() < widget.height(rect.size()) =>
            {
                self.cache.diff(small);
                small.render(&mut buffer, rect, &mut self.cache);
            }
            _ => {
                self.cache.diff(&widget);
                widget.render(&mut buffer, rect, &mut self.cache);
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
        let (w, h) = Self::get_size().ok_or(Error::UnknownTerminalSize)?;

        let pos = Vec2::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Vec2::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );

        if size != self.last_size {
            self.clear_cache();
            self.last_size = size;
        }

        Ok(Rect::from_coords(pos, size))
    }
}

impl Default for Term<DefaultBackend> {
    fn default() -> Self {
        Self {
            backend: Default::default(),
            prev: Default::default(),
            prev_widget: Default::default(),
            small: Default::default(),
            cache: Default::default(),
            padding: Default::default(),
            setuped: Default::default(),
            last_size: Default::default(),
        }
    }
}

impl<B> Drop for Term<B> {
    fn drop(&mut self) {
        if self.setuped {
            print!("{}{}", DISABLE_ALTERNATIVE_BUFFER, SHOW_CURSOR);
            _ = stdout().flush();
            _ = disable_raw_mode();
            self.setuped = false;
        }
    }
}

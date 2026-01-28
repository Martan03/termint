use std::io::{stdout, Write};

use termal::{
    codes::{
        DISABLE_ALTERNATIVE_BUFFER, ENABLE_ALTERNATIVE_BUFFER, ERASE_SCREEN,
        HIDE_CURSOR, SHOW_CURSOR,
    },
    raw::{
        disable_raw_mode, enable_raw_mode, term_size, StdioProvider, Terminal,
    },
};

use crate::{
    buffer::Buffer,
    error::Error,
    geometry::{Padding, Rect, Vec2},
    term::{Action, Application, Frame},
    widgets::{cache::Cache, Element, Widget},
};

/// The main entry points for terminal management and rendering.
///
/// [`Term`] provides two ways to build the TUI:
/// 1. **Framework mode**: using [`Term::run`] with [`Application`] trait
///     (recommended).
/// 2. **Manual mode**: manually managing the application lifetime.
///
/// # Example (framework mode):
///
/// Simple app definition and usage without any event handling (results in
/// static app).
///
/// ```rust
/// # use termint::{
/// #     term::{Term, Application, Frame},
/// #     widgets::Element
/// # };
/// struct MyApp;
/// impl Application for MyApp {
///     fn view(&self, _frame: &Frame) -> Element {
///         "Your UI here".into()
///     }
/// }
/// # fn example() -> Result<(), termint::Error> {
/// let mut app = MyApp;
/// Term::new().run(&mut app)?;
/// # Ok(())
/// # }
/// ```
///
/// # Example (manual mode):
///
/// ```rust
/// # use termint::{
/// #    term::Term, widgets::{Block, ToSpan}
/// # };
///
/// # fn example() -> Result<(), termint::Error> {
/// let main = Block::vertical().title("Example".to_span());
/// // Creates new Term with padding 1 on every side
/// let mut term = Term::new().padding(1);
/// term.render(main)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Term {
    prev: Option<Buffer>,
    prev_widget: Option<Element>,
    small: Option<Element>,
    cache: Cache,
    padding: Padding,
    setuped: bool,
}

impl Term {
    /// Creates new [`Term`] with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepares the terminal: enables the alternate buffer, clears screen,
    /// hides cursor and enable raw mode.
    ///
    /// When using manualy rendering ([`Term::render`] or [`Term::draw`]), you
    /// should call this once at the start of your program.
    ///
    /// The terminal is restored automatically when [`Term`] is dropped.
    pub fn setup(&mut self) -> Result<(), Error> {
        if !self.setuped {
            enable_raw_mode()?;
            print!(
                "{}{}{}",
                ENABLE_ALTERNATIVE_BUFFER, ERASE_SCREEN, HIDE_CURSOR
            );
            _ = stdout().flush();
            self.setuped = true;
        }
        Ok(())
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
    /// # use termint::{
    /// #    term::Term, widgets::{Block, ToSpan}
    /// # };
    ///
    /// # fn example() -> Result<(), termint::Error> {
    /// let main = Block::vertical().title("Example".to_span());
    /// // Creates new Term with padding 1 on every side
    /// let mut term = Term::new().padding(1);
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

    /// Starts the application main loop and handles the terminal state.
    ///
    /// This method does the following:
    /// 1. Calls [`Term::setup`] to setup terminal and does the initial render
    /// 2. Main loop: polls for events and updates the state:
    ///     - Calls [`Application::event`] on event
    ///     - Calls [`Application::update`] each tick
    ///     - Runs corresponding merged action from previous calls
    /// 3. Ends the main loop when [`Action::QUIT`] is received
    ///
    /// # Example
    ///
    /// ```rust
    /// # use termint::{
    /// #     term::{Term, Application, Frame},
    /// #     widgets::{Spacer, Element}
    /// # };
    /// # #[derive(Default)]
    /// # struct MyApp;
    /// # impl Application for MyApp {
    /// #     fn view(&self, _frame: &Frame) -> Element {
    /// #         Spacer::new().into()
    /// #     }
    /// # }
    /// # fn example() -> Result<(), termint::Error> {
    /// let mut term = Term::new();
    /// let mut app = MyApp::default();
    /// term.run(&mut app)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn run<A: Application>(&mut self, app: &mut A) -> Result<(), Error> {
        self.setup()?;
        let mut term = Terminal::<StdioProvider>::default();
        self.draw(|f| app.view(f))?;

        let timeout = app.poll_timeout();
        loop {
            let mut action = Action::NONE;
            if let Some(event) = term.read_timeout(timeout)? {
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

    /// Clears the cache of the [`Term`].
    ///
    /// This is useful when a widget's state changes, but the cache doesn't
    /// automatically update. After clearing the cache, the next rendering will
    /// recalculate the sizes and positions of widgets.
    pub fn clear_cache(&mut self) {
        self.cache = Cache::default();
    }

    /// Gets size of the terminal
    pub fn get_size() -> Option<(usize, usize)> {
        term_size().ok().map(|s| (s.char_width, s.char_height))
    }
}

impl Term {
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

    fn get_rect(&self) -> Result<Rect, Error> {
        let (w, h) = Term::get_size().ok_or(Error::UnknownTerminalSize)?;

        let pos = Vec2::new(1 + self.padding.left, 1 + self.padding.top);
        let size = Vec2::new(
            w.saturating_sub(self.padding.get_horizontal()),
            h.saturating_sub(self.padding.get_vertical()),
        );
        let rect = Rect::from_coords(pos, size);
        Ok(rect)
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if self.setuped {
            print!("{}{}", DISABLE_ALTERNATIVE_BUFFER, SHOW_CURSOR);
            _ = stdout().flush();
            _ = disable_raw_mode();
        }
    }
}

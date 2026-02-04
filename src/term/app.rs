use std::time::Duration;

use crate::{
    term::{backend::Event, Action, Frame},
    widgets::Element,
};

pub trait Application {
    type Message;

    /// Returns the widget tree to be rendered.
    ///
    /// This is called by [`crate::term::Term`] whenever [`Action::RENDER`] is
    /// triggered. See [`Frame`] documentation to know what information it
    /// contains.
    fn view(&self, frame: &Frame) -> Element<Self::Message>;

    /// Handles terminal events such as key presses.
    ///
    /// It's used to update internal state and return [`Action`] to signal if
    /// the UI needs to be updated. See [`Action`] documentation to know all
    /// the variants and their meanings.
    fn event(&mut self, _event: Event) -> Action {
        Action::NONE
    }

    /// Handles the message events thrown by the widgets mouse events.
    ///
    /// Some widgets support adding a mouse handler (such as Click, Scroll,...)
    /// and when this mouse event occurs, the widget then returns the set
    /// Message.
    fn message(&mut self, _message: Self::Message) -> Action {
        Action::NONE
    }

    /// Called every loop iteration, regardless of user input.
    ///
    /// This is ideal for animations, background taks or timer related logic.
    /// Return [`Action`] to signal, if the UI needs to be updated. See
    /// [`Action`] documentation to know all the variants and their meanings.
    fn update(&mut self) -> Action {
        Action::NONE
    }

    /// Returns the maximum duration to wait for an event before calling
    /// [`Self::update`].
    ///
    /// Apps that need higher refresh rate (such as for animations), should
    /// set shorter duration, such as 16ms which is around 60 FPS. Static apps
    /// can use shorter duration.
    fn poll_timeout(&self) -> Duration {
        Duration::from_millis(100)
    }
}

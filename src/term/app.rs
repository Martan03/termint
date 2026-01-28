use std::time::Duration;

use termal::raw::events::Event;

use crate::{
    term::{Action, Frame},
    widgets::Element,
};

pub trait Application {
    /// Returns the widget tree to be rendered.
    ///
    /// This is called by [`Term`] whenever [`Action::RENDER`] is triggered.
    /// See [`Frame`] documentation to know what information it contains.
    fn view(&self, frame: &Frame) -> Element;

    /// Handles terminal events such as key presses.
    ///
    /// It's used to update internal state and return [`Action`] to signal if
    /// the UI needs to be updated. See [`Action`] documentation to know all
    /// the variants and their meanings.
    fn event(&mut self, _event: Event) -> Action {
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

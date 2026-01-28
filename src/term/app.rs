use std::time::Duration;

use termal::raw::events::Event;

use crate::{
    term::{Action, Frame},
    widgets::Element,
};

pub trait Application {
    /// Returns the widget tree to be rendered. Called when `Action::Render`
    /// is given from either `event` or `update`.
    fn view(&self, frame: &Frame) -> Element;

    /// Handles terminal events (input). Returns an [`Action`].
    fn event(&mut self, _event: Event) -> Action {
        Action::NONE
    }

    /// Called every loop iteration (for animations, timers, etc.).
    fn update(&mut self) -> Action {
        Action::NONE
    }

    fn poll_timeout(&self) -> Duration {
        Duration::from_millis(100)
    }
}

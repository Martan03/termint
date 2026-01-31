use crate::{
    term::backend::{Backend, Event},
    Error,
};

pub struct CrosstermBackend;

impl Backend for CrosstermBackend {
    fn read_event(
        &mut self,
        timeout: std::time::Duration,
    ) -> Result<Option<Event>, Error> {
        todo!()
    }
}

use termal::raw::{StdioProvider, Terminal};

use crate::{
    term::backend::{Backend, Event},
    Error,
};

pub struct TermalBackend(pub(crate) Terminal<StdioProvider>);

impl Backend for TermalBackend {
    fn read_event(
        &mut self,
        timeout: std::time::Duration,
    ) -> Result<Option<Event>, Error> {
        todo!()
    }
}

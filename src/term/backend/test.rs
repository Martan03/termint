use std::time::Duration;

use crate::{prelude::Vec2, term::backend::Backend};

#[derive(Debug, Clone)]
pub struct TestBackend {
    size: Vec2,
}

impl TestBackend {
    pub fn new(size: impl Into<Vec2>) -> Self {
        Self { size: size.into() }
    }
}

impl Default for TestBackend {
    fn default() -> Self {
        Self {
            size: Vec2::new(80, 24),
        }
    }
}

impl Backend for TestBackend {
    fn read_event(
        &mut self,
        _timeout: Duration,
    ) -> Result<Option<super::Event>, crate::Error> {
        Ok(None)
    }

    fn enable_raw_mode() -> Result<(), crate::Error> {
        Ok(())
    }

    fn disable_raw_mode() -> Result<(), crate::Error> {
        Ok(())
    }

    fn is_raw_mode_enabled() -> bool {
        false
    }

    fn get_size(&self) -> Result<(usize, usize), crate::Error> {
        Ok((self.size.x, self.size.y))
    }
}

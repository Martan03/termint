use std::{io::Write, time::Duration};

use termal::raw::{disable_raw_mode, enable_raw_mode};
use termint::{
    prelude::*,
    term::{
        backend::Backend, disable_bracketed_paste, disable_mouse_capture,
        enable_bracketed_paste, enable_mouse_capture,
    },
};

fn main() -> Result<(), Error> {
    enable_raw_mode()?;
    let mut backend = CrosstermBackend::default();

    enable_bracketed_paste();
    enable_mouse_capture();
    let mut stdout = std::io::stdout();

    let timeout = Duration::from_millis(100);
    loop {
        if let Some(event) = backend.read_event(timeout)? {
            match event {
                Event::Key(key)
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL) =>
                {
                    break
                }
                _ => {
                    print!("{:?}\n\r", event);
                    _ = stdout.flush();
                }
            }
        }
    }
    disable_bracketed_paste();
    disable_mouse_capture();
    disable_raw_mode()?;
    Ok(())
}

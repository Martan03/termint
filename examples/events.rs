use std::{io::Write, time::Duration};

use termint::{
    prelude::*,
    term::{
        backend::{Backend, DefaultBackend},
        disable_bracketed_paste, disable_mouse_capture,
        enable_bracketed_paste, enable_mouse_capture,
    },
};

fn main() -> Result<(), Error> {
    DefaultBackend::enable_raw_mode()?;
    let mut backend = DefaultBackend::default();

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
                    break;
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
    DefaultBackend::disable_raw_mode()?;
    Ok(())
}

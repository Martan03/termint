use std::{cell::Cell, process::ExitCode, rc::Rc, time::Duration};

use termal::{
    eprintcln,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Key, KeyCode},
        StdioProvider, Terminal,
    },
};
use termint::{
    enums::{BorderType, Color},
    geometry::Constraint,
    style::Style,
    term::Term,
    widgets::{Block, ProgressBar, Spacer, ToSpan},
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELECTED: Color = Color::Hex(0xea4bfc);

fn main() -> ExitCode {
    if let Err(e) = App::run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

struct App {
    term: Term,
    state: Rc<Cell<f64>>,
}

impl App {
    pub fn run() -> termal::error::Result<()> {
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = enable_raw_mode();

        let mut app = App::default();
        let mut term = Terminal::<StdioProvider>::default();
        app.render();

        let timeout = Duration::from_millis(50);
        loop {
            if let Some(event) = term.read_timeout(timeout)? {
                match event {
                    Event::KeyPress(key) => {
                        if app.key_listener(key) {
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let mut state = app.state.get() + 1.;
            if state > 120. {
                state = 0.;
            }
            app.state.set(state);
            _ = app.term.rerender();
        }

        _ = disable_raw_mode();
        print!("\x1b[?1049l\x1b[?25h");
        Ok(())
    }

    fn render(&mut self) {
        let bar = ProgressBar::new(self.state.clone());
        let help = "[Esc|q]Quit".fg(BORDER);

        let mut block = Block::vertical()
            .title("Progress Bar")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        block.push(bar, Constraint::Min(0));
        block.push(Spacer::new(), Constraint::Fill(1));
        block.push(help, 1..);

        _ = self.term.render(block);
    }

    fn key_listener(&mut self, key: Key) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return true,
            _ => return false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            term: Term::new(),
            state: Rc::new(Cell::new(50.)),
        }
    }
}

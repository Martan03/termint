use std::{cell::Cell, process::ExitCode, rc::Rc, time::Duration};

use termal::{
    eprintcln,
    raw::events::{Event, Key, KeyCode},
};
use termint::{
    enums::{BorderType, Color},
    geometry::Constraint,
    style::Style,
    term::{Action, Application, Frame, Term},
    widgets::{Block, Element, ProgressBar, Spacer, ToSpan},
    Error,
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Error> {
    let mut term = Term::new();
    let mut app = App::default();
    term.run(&mut app)
}

struct App {
    states: Vec<Rc<Cell<f64>>>,
}

impl Application for App {
    fn view(&self, _frame: &Frame) -> Element {
        let mut block = Block::vertical()
            .title("Progress Bar")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        for state in self.states.iter() {
            let bar = ProgressBar::new(state.clone());
            block.push(bar, Constraint::Min(0));
            block.push(Spacer::new(), 1);
        }

        block.push(Spacer::new(), Constraint::Fill(1));
        let help = "[Esc|q]Quit".fg(BORDER);
        block.push(help, 1..);
        block.into()
    }

    fn event(&mut self, event: Event) -> Action {
        match event {
            Event::KeyPress(key) => self.key_listener(key),
            _ => Action::NONE,
        }
    }

    fn update(&mut self) -> Action {
        if self.increase_states() {
            self.reset_states();
        }
        Action::RERENDER
    }

    fn poll_timeout(&self) -> Duration {
        Duration::from_millis(50)
    }
}

impl App {
    fn key_listener(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Action::QUIT,
            _ => Action::NONE,
        }
    }

    fn increase_states(&mut self) -> bool {
        let len = self.states.len() as f64;

        let mut complete = true;
        for (i, state) in self.states.iter().enumerate() {
            let speed = (len - i as f64) / len;
            let val = state.get() + speed * 3.;
            state.set(val);
            if val < 120. {
                complete = false;
            }
        }
        complete
    }

    fn reset_states(&mut self) {
        for state in self.states.iter() {
            state.set(0.);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            states: (0..5).map(|_| Rc::new(Cell::new(0.))).collect(),
        }
    }
}

use std::{cell::Cell, process::ExitCode, rc::Rc, time::Duration};

use termal::eprintcln;
use termint::prelude::*;

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
    let mut app = App::default();
    Term::default().setup()?.with_mouse().run(&mut app)
}

#[derive(Clone)]
enum Message {
    Seek(usize, f64),
}

struct App {
    states: Vec<Rc<Cell<f64>>>,
    running: bool,
}

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let mut block = Block::vertical()
            .title("Progress Bar")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let tasks = [
            ("Fetching Repo", Color::Hex(0xF4F8D3)),
            ("Downloading deps", Color::Hex(0xA6D6D6)),
            ("Compiling termint", Color::Hex(0xF7CFD8)),
            ("Running tests", Color::Hex(0x8E7DBE)),
            ("Building docs", Color::Hex(0xB0DB9C)),
        ];

        let mut label_layout = Layout::vertical();
        let mut pb_layout = Layout::vertical();
        for (i, state) in self.states.iter().enumerate() {
            let (name, color) = tasks[i];

            label_layout.push(name.fg(color).bold(), 1);
            label_layout.push(Spacer, 1);

            let bar = ProgressBar::new(state.clone())
                .style(Style::new().bg(color).fg(color))
                .on_click(move |p| Message::Seek(i, p));
            pb_layout.push(bar, 1);
            pb_layout.push(Spacer, 1);
        }

        let mut wrapper = Layout::horizontal();
        wrapper.push(label_layout, 0..);
        wrapper.push(Spacer, 1);
        wrapper.push(pb_layout, 0..);

        block.push(wrapper, 0..);
        block.push(Spacer, Constraint::Fill(1));
        let help =
            "[Click] Seek Bar  [Space] Play/pause  [r] Reset  [Esc|q] Quit"
                .fg(BORDER);
        block.push(help, 1..);
        block.into()
    }

    fn event(&mut self, event: Event) -> Action {
        match event {
            Event::Key(key) => self.key_listener(key),
            _ => Action::NONE,
        }
    }

    fn message(&mut self, message: Self::Message) -> Action {
        match message {
            Message::Seek(id, p) => self.states[id].set(p),
        }
        Action::RERENDER
    }

    fn update(&mut self, delta: Duration) -> Action {
        if !self.running {
            return Action::NONE;
        }

        if self.increase_states(delta) {
            self.reset_states();
        }
        Action::RERENDER
    }

    fn poll_timeout(&self) -> Duration {
        // This doesn't wait for event, so the animation is really smooth :)
        Duration::from_millis(0)
    }
}

impl App {
    fn key_listener(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char(' ') => {
                self.running = !self.running;
                Action::NONE
            }
            KeyCode::Char('r') => {
                self.reset_states();
                Action::RERENDER
            }
            KeyCode::Esc | KeyCode::Char('q') => Action::QUIT,
            _ => Action::NONE,
        }
    }

    fn increase_states(&mut self, delta: Duration) -> bool {
        let len = self.states.len() as f64;

        let mut complete = true;
        for (i, state) in self.states.iter().enumerate() {
            let speed = (len - i as f64) / len;
            let val = state.get() + speed * delta.as_secs_f64() * 0.5;
            state.set(val);
            if val < 1.20 {
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
            running: true,
        }
    }
}

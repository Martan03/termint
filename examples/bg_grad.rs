use std::process::ExitCode;

use termal::{
    eprintcln,
    raw::events::{Event, Key, KeyCode},
};
use termint::{
    enums::{BorderType, Color},
    geometry::{Constraint, Direction},
    style::Style,
    term::{Action, Application, Frame, Term},
    widgets::{BgGrad, Block, Element, Layout, Spacer, ToSpan},
    Error,
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);

const COLORS: [((u8, u8, u8), (u8, u8, u8)); 3] = [
    ((0, 220, 255), (175, 80, 255)),
    ((255, 255, 0), (0, 255, 255)),
    ((255, 15, 123), (248, 155, 41)),
];

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
    dir: Direction,
    cur: usize,
}

impl Application for App {
    fn view(&self, _frame: &Frame) -> Element {
        let mut block = Block::vertical()
            .title("Centering")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let mut center = Layout::horizontal().center();
        let (start, end) = COLORS[self.cur];
        center.push(BgGrad::new(self.dir, start, end), 50);

        block.push(Spacer::new(), Constraint::Fill(1));
        block.push(center, 25);
        block.push(Spacer::new(), Constraint::Fill(1));
        let help = "[←]Prev. grad. [→]Next grad. [r]Rotate grad. [Esc|q]Quit"
            .fg(BORDER);
        block.push(help, 1..);
        block.into()
    }

    fn event(&mut self, event: Event) -> Action {
        match event {
            Event::KeyPress(key) => self.key_listener(key),
            _ => Action::NONE,
        }
    }
}

impl App {
    fn key_listener(&mut self, key: Key) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            KeyCode::Left => {
                self.cur = self.cur.checked_sub(1).unwrap_or(COLORS.len() - 1)
            }
            KeyCode::Right => self.cur = (self.cur + 1) % COLORS.len(),
            KeyCode::Char('r') => self.toggle_dir(),
            _ => return Action::NONE,
        }
        Action::RENDER
    }

    fn toggle_dir(&mut self) {
        match self.dir {
            Direction::Vertical => self.dir = Direction::Horizontal,
            Direction::Horizontal => self.dir = Direction::Vertical,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            dir: Direction::Horizontal,
            cur: 0,
        }
    }
}

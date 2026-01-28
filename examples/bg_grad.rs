use std::{process::ExitCode, time::Duration};

use termal::{
    eprintcln,
    raw::{
        events::{Event, Key, KeyCode},
        StdioProvider, Terminal,
    },
};
use termint::{
    enums::{BorderType, Color},
    geometry::{Constraint, Direction},
    style::Style,
    term::Term,
    widgets::{BgGrad, Block, Layout, Spacer, ToSpan},
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
    if let Err(e) = App::run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

struct App {
    term: Term,
    dir: Direction,
    cur: usize,
}

impl App {
    pub fn run() -> Result<(), Error> {
        let mut app = App::default();
        app.term.setup()?;

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
        }
        Ok(())
    }

    fn render(&mut self) {
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

        _ = self.term.render(block);
    }

    fn key_listener(&mut self, key: Key) -> bool {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return true,
            KeyCode::Left => {
                self.cur = self.cur.checked_sub(1).unwrap_or(COLORS.len() - 1)
            }
            KeyCode::Right => self.cur = (self.cur + 1) % COLORS.len(),
            KeyCode::Char('r') => self.toggle_dir(),
            _ => return false,
        }
        self.render();
        false
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
            term: Term::new(),
            dir: Direction::Horizontal,
            cur: 0,
        }
    }
}

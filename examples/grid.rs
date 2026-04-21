use std::process::ExitCode;

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
    Term::default().setup()?.run(&mut app)
}

#[derive(Default)]
struct App;

impl Application for App {
    type Message = ();

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let mut block = Block::vertical()
            .title("Grid Example")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let mut grid = Grid::new(
            [Unit::Fill(1), Unit::Length(30), Unit::Fill(1)],
            [Unit::Length(3), Unit::Fill(1), Unit::Length(1)],
        );

        let mut header = Block::horizontal().center();
        header.push("Column span of 3", 0..);
        grid.push_span(header, 0, 0, 3, 1);

        block.push(grid, Constraint::Fill(1));
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
}

impl App {
    fn key_listener(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Action::QUIT,
            _ => Action::NONE,
        }
    }
}

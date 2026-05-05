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
        let bstyle = Style::new().bg(BG).fg(BORDER);
        let mut block = Block::vertical()
            .title("Grid Example")
            .border_type(BorderType::Thicker)
            .border_style(bstyle)
            .style(Style::new().bg(BG).fg(FG));

        let mut grid = Grid::new(
            [Unit::Fill(1), Unit::Percent(50), Unit::Fill(1)],
            [Unit::Length(3), Unit::Fill(1), Unit::Length(3)],
        );

        let mut header = Block::horizontal().center().border_style(bstyle);
        header.push("Header (col_span: 3)", 0..);
        grid.push_span(header, 0, 0, 3, 1);

        let mut menu = Block::vertical().center().border_style(bstyle);
        menu.push("Menu (row_span: 2)".align(TextAlign::Center), 0..);
        grid.push_span(menu, 0, 1, 1, 2);

        let mut main = Block::vertical().center().border_style(bstyle);
        main.push("Content".align(TextAlign::Center), 0..);
        grid.push(main, 1, 1);

        let mut right = Block::vertical().center().border_style(bstyle);
        right.push("Sidebar".align(TextAlign::Center), 0..);
        grid.push(right, 2, 1);

        let mut footer = Block::horizontal().center().border_style(bstyle);
        footer.push("Footer (col_span: 2)", 0..);
        grid.push_span(footer, 1, 2, 2, 1);

        block.push(grid, Constraint::Fill(1));
        let help = "[Esc|q] Quit".fg(BORDER);
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

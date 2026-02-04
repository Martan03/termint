use std::process::ExitCode;

use termal::eprintcln;
use termint::{
    enums::{BorderType, Color},
    geometry::Constraint,
    style::Style,
    term::{
        backend::{Event, KeyCode, KeyEvent},
        Action, Application, Frame, Term,
    },
    widgets::{Block, Button, Element, Layout, Spacer, ToSpan},
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
    let mut app = App::default();
    Term::default().setup()?.with_mouse().run(&mut app)
}

#[derive(Clone)]
enum Message {
    Quit,
}

#[derive(Default)]
struct App;

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let mut block = Block::vertical()
            .title("Centering")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let button = Button::new("Quit App")
            .style(Style::new().bg(Color::Cyan).fg(BG))
            .padding((1, 2))
            .on_click(Message::Quit);
        let mut wrapper = Layout::vertical();
        wrapper.push(button, 0..);

        let mut center = Layout::horizontal().center();
        center.push(wrapper, 0..);

        block.push(Spacer::new(), Constraint::Fill(1));
        block.push(center, 0..);
        block.push(Spacer::new(), Constraint::Fill(1));
        let help = "[Esc|q]Quit".fg(BORDER);
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
            Message::Quit => Action::QUIT,
        }
    }
}

impl App {
    fn key_listener(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            _ => Action::NONE,
        }
    }
}

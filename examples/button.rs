use std::process::ExitCode;

use termal::eprintcln;
use termint::{
    Error,
    enums::{BorderType, Color},
    geometry::Constraint,
    prelude::TextAlign,
    style::{Style, Stylize},
    term::{
        Action, Application, Frame, Term,
        backend::{Event, KeyCode, KeyEvent, MouseButton},
    },
    widgets::{Block, Button, Element, Layout, Spacer, ToSpan},
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const INC: Color = Color::Hex(0xA3DC9A);
const DEC: Color = Color::Hex(0xEA7B7B);
const BTN: Color = Color::Hex(0x98A1BC);

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
    Increment,
    Decrement,
    Reset,
    Quit,
}

#[derive(Default)]
struct App {
    count: isize,
}

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let mut block = Block::vertical()
            .title("Button Example")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let counter = format!("Counter: {}", self.count).bold();

        let inc_btn = Button::new("+".align(TextAlign::Center))
            .style((BG, INC))
            .padding((1, 2))
            .on_click(Message::Increment);
        let dec_btn = Button::new("-".align(TextAlign::Center))
            .style((BG, DEC))
            .padding((1, 2))
            .on_click(Message::Decrement);

        let btn = Button::new("Left = Reset, Right = Quit")
            .style((BG, BTN))
            .padding((1, 2))
            .on_click(Message::Reset)
            .on_press(MouseButton::Right, Message::Quit);

        let mut buttons = Layout::horizontal().center();
        buttons.push(inc_btn, 11);
        buttons.push(Spacer::new(), 8);
        buttons.push(dec_btn, 11);

        let mut wrapper = Layout::vertical();
        wrapper.push(counter, 0..);
        wrapper.push(Spacer::new(), 1);
        wrapper.push(buttons, 0..);
        wrapper.push(Spacer::new(), 1);
        wrapper.push(btn, 0..);

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
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
            Message::Quit => return Action::QUIT,
        }
        Action::RENDER
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

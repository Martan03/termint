use std::{cell::Cell, process::ExitCode, rc::Rc};

use termal::eprintcln;
use termint::{
    Error,
    enums::{Border, BorderType, Color, Modifier},
    geometry::Constraint,
    style::{Style, Stylize},
    term::{
        Action, Application, Frame, Term,
        backend::{Event, KeyCode, KeyEvent},
    },
    widgets::{Block, Element, Layout, Scrollable, ScrollbarState, ToSpan},
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SEL: Color = Color::Hex(0xea4bfc);

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

struct App {
    ver_state: Rc<Cell<ScrollbarState>>,
    hor_state: Rc<Cell<ScrollbarState>>,
}

impl Application for App {
    type Message = ();

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let mut layout = Layout::vertical();
        for i in 0..20 {
            layout.push(
                format!("Title {i}").modifier(Modifier::UNDERLINED).fg(SEL),
                0..,
            );
            let block = Block::new(get_lorem()).borders(Border::LEFT);
            layout.push(block, 2..);
        }

        let scrollable: Scrollable<_, Block> = Scrollable::both(
            layout,
            self.ver_state.clone(),
            self.hor_state.clone(),
        );
        let help = "[↑]Move up [↓]Move down [Esc|q]Quit".fg(BORDER);

        let mut block = Block::vertical()
            .title("Scrollable Example")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(scrollable, Constraint::Fill(1));
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
            KeyCode::Down => self.ver_state.set(self.ver_state.get().next()),
            KeyCode::Up => self.ver_state.set(self.ver_state.get().prev()),
            KeyCode::Right => self.hor_state.set(self.hor_state.get().next()),
            KeyCode::Left => self.hor_state.set(self.hor_state.get().prev()),
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            _ => return Action::NONE,
        }
        Action::RERENDER
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            ver_state: Rc::new(Cell::new(ScrollbarState::new(0))),
            hor_state: Rc::new(Cell::new(ScrollbarState::new(0))),
        }
    }
}

fn get_lorem() -> String {
    "Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus \
    ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus \
    duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar \
    vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl \
    malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class \
    aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos \
    himenaeos."
        .to_string()
}

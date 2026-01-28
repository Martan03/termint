use std::{cell::RefCell, process::ExitCode, rc::Rc};

use termal::{
    eprintcln,
    raw::events::{Event, Key, KeyCode},
};
use termint::{
    enums::{BorderType, Color},
    geometry::Constraint,
    style::Style,
    term::{Action, Application, Frame, Term},
    widgets::{Block, Element, List, ListState, ToSpan},
    Error,
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELECTED: Color = Color::Hex(0xea4bfc);

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Error> {
    let mut app = App::default();
    Term::new().run(&mut app)
}

struct App {
    list_state: Rc<RefCell<ListState>>,
    people: Vec<String>,
}

impl Application for App {
    fn view(&self, _frame: &Frame) -> Element {
        let list = List::new(&self.people, self.list_state.clone())
            .auto_scroll()
            .selected_style(SELECTED)
            .scrollbar_fg(BORDER)
            .thumb_fg(FG);
        let help = "[↑]Move up [↓]Move down [Esc|q]Quit".fg(BORDER);

        let mut block = Block::vertical()
            .title("Quest List")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(list, Constraint::Fill(1));
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
            KeyCode::Down => {
                let mut state = self.list_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return Action::NONE;
                };

                if sel + 1 < self.people.len() {
                    state.selected = Some(sel + 1);
                }
            }
            KeyCode::Up => {
                let mut state = self.list_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return Action::NONE;
                };

                state.selected = Some(sel.saturating_sub(1));
            }
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            _ => return Action::NONE,
        }
        Action::RERENDER
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            list_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
            people: get_people(),
        }
    }
}

fn get_people() -> Vec<String> {
    vec![
        "Alice Johnson".to_string(),
        "Bob Smith".to_string(),
        "Carol Davis".to_string(),
        "David Thompson".to_string(),
        "Emma Wilson".to_string(),
        "Frank Miller".to_string(),
        "Grace Lee".to_string(),
        "Henry Clark".to_string(),
        "Isla Lewis".to_string(),
        "Jack Martin".to_string(),
    ]
}

use std::{cell::RefCell, process::ExitCode, rc::Rc};

use fake::Fake;
use termal::eprintcln;
use termint::{prelude::*, text::Text};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELECTED: Color = Color::Hex(0xea4bfc);
const HIGHLIGHT: Color = Color::Hex(0x891296);

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
pub enum Message {
    Select(usize),
}

struct App {
    list_state: Rc<RefCell<ListState>>,
    people: Vec<(String, String)>,
}

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let items: Vec<Box<dyn Text>> = self
            .people
            .iter()
            .enumerate()
            .map(|(i, (n, d))| {
                let mut text = Paragraph::empty();
                text.push(format!("{:02}.", i + 1).fg(BORDER));
                text.push(n.clone().fg(Color::White));
                text.push(format!("\n  - {}", d));
                text.into()
            })
            .collect();

        let list = List::new(items, self.list_state.clone())
            .auto_scroll()
            .selected_style(SELECTED)
            .scrollbar_fg(BORDER)
            .highlight_symbol(" > ")
            .highlight_style(HIGHLIGHT)
            .on_click(Message::Select)
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
            Event::Key(key) => self.key_listener(key),
            _ => Action::NONE,
        }
    }

    fn message(&mut self, message: Self::Message) -> Action {
        match message {
            Message::Select(id) => self.select(id),
        }
        Action::RERENDER
    }
}

impl App {
    fn key_listener(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let mut state = self.list_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return Action::NONE;
                };

                if sel + 1 < self.people.len() {
                    state.selected = Some(sel + 1);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
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

    fn select(&self, id: usize) {
        let mut state = self.list_state.borrow_mut();
        state.selected = Some(id);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            list_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
            people: get_people(100),
        }
    }
}

fn get_people(count: usize) -> Vec<(String, String)> {
    (1..count)
        .map(|_| {
            (
                fake::faker::name::en::Name().fake(),
                fake::faker::lorem::en::Sentence(3..8).fake(),
            )
        })
        .collect()
}

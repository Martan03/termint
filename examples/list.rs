use std::{cell::RefCell, process::ExitCode, rc::Rc, time::Duration};

use termal::{
    eprintcln,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Key, KeyCode},
        StdioProvider, Terminal,
    },
};
use termint::{
    enums::{BorderType, Color},
    geometry::Constraint,
    style::Style,
    term::Term,
    widgets::{Block, List, ListState, ToSpan},
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELECTED: Color = Color::Hex(0xea4bfc);

fn main() -> ExitCode {
    if let Err(e) = App::run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

struct App {
    term: Term,
    list_state: Rc<RefCell<ListState>>,
    people: Vec<String>,
}

impl App {
    pub fn run() -> termal::error::Result<()> {
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = enable_raw_mode();

        let mut app = App::default();
        let mut term = Terminal::<StdioProvider>::default();
        app.render();

        let timeout = Duration::from_millis(100);
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

        _ = disable_raw_mode();
        print!("\x1b[?1049l\x1b[?25h");
        Ok(())
    }

    fn render(&mut self) {
        let list = List::new(&self.people, self.list_state.clone())
            .auto_scroll()
            .selected_style(SELECTED)
            .scrollbar_fg(BORDER)
            .thumb_fg(FG);
        let help = "[↑]Move up [↓]Move down [Esc]Quit".fg(BORDER);

        let mut block = Block::vertical()
            .title("Quest List")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(list, Constraint::Fill(1));
        block.push(help, 1..);

        _ = self.term.render(block);
    }

    fn key_listener(&mut self, key: Key) -> bool {
        match key.code {
            KeyCode::Down => {
                let mut state = self.list_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return false;
                };

                if sel + 1 < self.people.len() {
                    state.selected = Some(sel + 1);
                }
            }
            KeyCode::Up => {
                let mut state = self.list_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return false;
                };

                state.selected = Some(sel.saturating_sub(1));
            }
            KeyCode::Esc => return true,
            _ => return false,
        }
        self.render();
        return false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            term: Term::new(),
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

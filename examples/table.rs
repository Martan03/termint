use std::{cell::RefCell, process::ExitCode, rc::Rc, time::Duration};

use termal::{
    eprintcln,
    raw::{
        events::{Event, Key, KeyCode},
        StdioProvider, Terminal,
    },
};
use termint::{
    enums::{BorderType, Color},
    geometry::{Constraint, Unit},
    style::Style,
    term::Term,
    widgets::{Block, Row, Table, TableState, ToSpan},
    Error,
};

const BG: Color = Color::Hex(0x02081e);
const BGL: Color = Color::Hex(0x061038);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELL: Color = Color::Hex(0xf3a6fc);
const SEL: Color = Color::Hex(0xea4bfc);

fn main() -> ExitCode {
    if let Err(e) = App::run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

struct App {
    term: Term,
    table_state: Rc<RefCell<TableState>>,
    songs: Vec<Vec<&'static str>>,
}

impl App {
    pub fn run() -> Result<(), Error> {
        let mut app = App::default();
        app.term.setup()?;

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
        Ok(())
    }

    fn render(&mut self) {
        let table = Table::new(
            get_rows(),
            vec![Unit::Fill(1); 3],
            self.table_state.clone(),
        )
        .header(vec!["Title", "Artist", "Album"])
        .header_separator(BorderType::Normal)
        .selected_row_style(SELL)
        .selected_column_style(SELL)
        .selected_cell_style(SEL)
        .auto_scroll();

        let help =
            "[↑]Move up [↓]Move down [←]Move left [→]Move right [Esc|q]Quit"
                .fg(BORDER);

        let mut block = Block::vertical()
            .title("Songs List")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(table, Constraint::Fill(1));
        block.push(help, 1..);

        _ = self.term.render(block);
    }

    fn key_listener(&mut self, key: Key) -> bool {
        match key.code {
            KeyCode::Down => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return false;
                };

                if sel + 1 < self.songs.len() {
                    state.selected = Some(sel + 1);
                }
            }
            KeyCode::Up => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return false;
                };

                state.selected = Some(sel.saturating_sub(1));
            }
            KeyCode::Left => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected_column else {
                    return false;
                };

                state.selected_column = Some(sel.saturating_sub(1));
            }
            KeyCode::Right => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected_column else {
                    return false;
                };

                if sel + 1 < 3 {
                    state.selected_column = Some(sel + 1);
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => return true,
            _ => return false,
        }
        _ = self.term.rerender();
        return false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            term: Term::new(),
            table_state: Rc::new(RefCell::new(
                TableState::new(0).selected(0).selected_column(0),
            )),
            songs: get_songs(),
        }
    }
}

fn get_songs() -> Vec<Vec<&'static str>> {
    vec![
        vec!["Emptiness Machine", "Linkin Park", "From Zero"],
        vec!["Numb", "Linkin Park", "Meteora"],
        vec!["Radioactive", "Imagine Dragons", "Night Visions"],
        vec!["Believer", "Imagine Dragons", "Evolve"],
        vec!["Stressed Out", "Twenty One Pilots", "Blurryface"],
        vec!["Ride", "Twenty One Pilots", "Blurryface"],
        vec!["Counting Stars", "OneRepublic", "Native"],
        vec!["Secrets", "OneRepublic", "Waking Up"],
        vec!["High Hopes", "Panic! At The Disco", "Pray for the Wicked"],
        vec![
            "I Write Sins Not Tragedies",
            "Panic! At The Disco",
            "A Fever You Can't Sweat Out",
        ],
    ]
}

fn get_rows() -> Vec<Row> {
    let rows = get_songs()
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut row = Row::new(s);
            if i % 2 == 0 {
                row = row.style(Style::new().bg(BGL));
            }
            row
        })
        .collect();
    rows
}

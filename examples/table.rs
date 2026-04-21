use std::{cell::RefCell, process::ExitCode, rc::Rc};

use fake::Fake;
use termal::eprintcln;
use termint::prelude::*;

const BG: Color = Color::Hex(0x02081e);
const BGL: Color = Color::Hex(0x061038);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);
const SELL: Color = Color::Hex(0xf3a6fc);
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

#[derive(Clone)]
enum Message {
    CellClicked(usize, usize),
}

struct App {
    table_state: Rc<RefCell<TableState>>,
    employees: Vec<Employee>,
}

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let table = Table::new(
            get_rows(&self.employees),
            [
                Unit::Length(4),
                Unit::Fill(1),
                Unit::Fill(1),
                Unit::Length(10),
            ],
            self.table_state.clone(),
        )
        .header(vec!["ID", "Name", "Email", "Status"])
        .header_separator(BorderType::Normal)
        .footer(vec!["ID", "Name", "Email", "Status"])
        .footer_separator(BorderType::Normal)
        .selected_row_style((BG, SELL))
        .selected_column_style(SELL)
        .selected_cell_style((BGL, SEL))
        .column_spacing(2)
        .on_click(Message::CellClicked)
        .auto_scroll();

        let help =
            "[↑]Move up [↓]Move down [←]Move left [→]Move right [Esc|q]Quit"
                .fg(BORDER);

        let mut block = Block::vertical()
            .title("Employees")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(table, Constraint::Fill(1));
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
            Message::CellClicked(x, y) => {
                let mut state = self.table_state.borrow_mut();
                state.selected = Some(y);
                state.selected_column = Some(x);
            }
        }
        Action::RERENDER
    }
}

#[derive(Clone)]
struct Employee {
    id: usize,
    name: String,
    email: String,
    active: bool,
}

impl Employee {
    fn generate(count: usize) -> Vec<Self> {
        (0..count)
            .map(|i| Employee {
                id: i + 1,
                name: fake::faker::name::en::Name().fake(),
                email: fake::faker::internet::en::SafeEmail().fake(),
                active: (0..10).fake::<u8>() > 2,
            })
            .collect()
    }
}

impl App {
    fn key_listener(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return Action::NONE;
                };

                if sel + 1 < self.employees.len() {
                    state.selected = Some(sel + 1);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected else {
                    return Action::NONE;
                };

                state.selected = Some(sel.saturating_sub(1));
            }
            KeyCode::Left | KeyCode::Char('h') => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected_column else {
                    return Action::NONE;
                };

                state.selected_column = Some(sel.saturating_sub(1));
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let mut state = self.table_state.borrow_mut();
                let Some(sel) = state.selected_column else {
                    return Action::NONE;
                };

                if sel + 1 < 4 {
                    state.selected_column = Some(sel + 1);
                }
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
            table_state: Rc::new(RefCell::new(
                TableState::new(0).selected(0).selected_column(0),
            )),
            employees: Employee::generate(100),
        }
    }
}

fn get_rows<M: Clone>(data: &[Employee]) -> Vec<Row<M>> {
    let rows = data
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let status = if e.active {
                "\nActive\n".green()
            } else {
                "\nOffline\n".gray()
            };

            let mut row = Row::new(vec![
                format!("\n{}\n", e.id).to_span(),
                format!("\n{}\n", e.name).to_span(),
                format!("\n{}\n", e.email).to_span(),
                status.into(),
            ]);
            if i % 2 == 0 {
                row = row.style(Style::new().bg(BGL));
            }
            row
        })
        .collect();
    rows
}

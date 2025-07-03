use std::{cell::Cell, process::ExitCode, rc::Rc, time::Duration};

use termal::{
    eprintcln,
    raw::{
        disable_raw_mode, enable_raw_mode,
        events::{Event, Key, KeyCode},
        StdioProvider, Terminal,
    },
};
use termint::{
    enums::{Border, BorderType, Color, Modifier},
    geometry::Constraint,
    style::Style,
    term::Term,
    widgets::{Block, Layout, Scrollable, ScrollbarState, Span, ToSpan},
};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);

fn main() -> ExitCode {
    if let Err(e) = App::run() {
        eprintcln!("{'r}Error:{'_} {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

struct App {
    term: Term,
    state: Rc<Cell<ScrollbarState>>,
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
        let mut layout = Layout::vertical();
        for i in 0..5 {
            layout.push(
                format!("Title {i}").modifier(Modifier::UNDERLINED),
                0..,
            );
            let block: Block<Span> =
                Block::new(get_lorem()).borders(Border::LEFT);
            layout.push(block, 0..);
        }

        let scrollable =
            Scrollable::<Layout>::vertical(layout, self.state.clone());
        let help = "[↑]Move up [↓]Move down [Esc|q]Quit".fg(BORDER);

        let mut block = Block::vertical()
            .title("Scrollable Example")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));
        block.push(scrollable, Constraint::Fill(1));
        block.push(help, 1..);

        _ = self.term.render(block);
    }

    fn key_listener(&mut self, key: Key) -> bool {
        match key.code {
            KeyCode::Down => self.state.set(self.state.get().next()),
            KeyCode::Up => self.state.set(self.state.get().prev()),
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
            state: Rc::new(Cell::new(ScrollbarState::new(0))),
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

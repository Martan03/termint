use std::process::ExitCode;

use fake::{Fake, faker::lorem::en::Words};
use termal::eprintcln;
use termint::{prelude::*, widgets::Grad};

const BG: Color = Color::Hex(0x02081e);
const BORDER: Color = Color::Hex(0x535C91);
const FG: Color = Color::Hex(0xc3c1f4);

const COLORS: [((u8, u8, u8), (u8, u8, u8)); 3] = [
    ((0, 220, 255), (175, 80, 255)),
    ((255, 255, 0), (0, 255, 255)),
    ((255, 15, 123), (248, 155, 41)),
];

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

struct App {
    lorem: String,
    align: TextAlign,
    dir: Direction,
    grad_id: usize,
}

impl Application for App {
    type Message = ();

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        let (start_col, end_col) = COLORS[self.grad_id];

        let mut block = Block::vertical()
            .title("Text Example")
            .border_type(BorderType::Thicker)
            .border_style(Style::new().bg(BG).fg(BORDER))
            .style(Style::new().bg(BG).fg(FG));

        let span = "Span widget (static styling)"
            .bold()
            .fg(Color::Yellow)
            .align(self.align);
        block.push(span, 1..);

        let grad =
            Grad::new("Grad widget (gradient text)", start_col, end_col)
                .align(self.align);
        block.push(grad, 1..);

        let mut paragraph = Paragraph::empty().align(self.align);
        paragraph.push("Paragraphs can mix".fg(Color::Green));
        let grad = Grad::new("Gradients", start_col, end_col);
        paragraph.push(grad);
        paragraph.push("and");
        paragraph.push("Spans.".red().bold());
        block.push(paragraph, 0..);

        let header = Block::empty()
            .title("Paragraph with ellipsis".align(self.align))
            .border_style(Style::new().bg(BG).fg(BORDER));
        block.push(Spacer, 1);
        block.push(header, 1);

        let mut lorem = Paragraph::empty().align(self.align);
        lorem.push(
            Grad::new(self.lorem.as_str(), start_col, end_col)
                .direction(self.dir),
        );
        lorem.push(self.lorem.as_str().italic());

        let mut wrapper = Layout::horizontal();
        let (left, right) = match self.align {
            TextAlign::Left => (Constraint::Length(0), Constraint::Fill(1)),
            TextAlign::Center => (Constraint::Fill(1), Constraint::Fill(1)),
            TextAlign::Right => (Constraint::Fill(1), Constraint::Length(0)),
        };
        wrapper.push(Spacer, left);
        wrapper.push(lorem, ..50);
        wrapper.push(Spacer, right);
        block.push(wrapper, 8);

        block.push(Spacer, Constraint::Fill(1));
        let help = "[←/→]Cycle grad.  [r]Rotate grad.  [a]Align  [Esc/q] Quit"
            .fg(BORDER)
            .align(TextAlign::Center);
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
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            KeyCode::Left | KeyCode::Char('h') => {
                self.grad_id =
                    self.grad_id.checked_sub(1).unwrap_or(COLORS.len() - 1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.grad_id = (self.grad_id + 1) % COLORS.len();
            }
            KeyCode::Char('r') => self.toggle_dir(),
            KeyCode::Char('a') => self.toggle_align(),
            _ => return Action::NONE,
        }
        Action::RENDER
    }

    fn toggle_dir(&mut self) {
        self.dir = match self.dir {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        };
    }

    fn toggle_align(&mut self) {
        self.align = match self.align {
            TextAlign::Left => TextAlign::Center,
            TextAlign::Center => TextAlign::Right,
            TextAlign::Right => TextAlign::Left,
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            lorem: Words(35..36).fake::<Vec<String>>().join(" "),
            align: TextAlign::Left,
            dir: Direction::Horizontal,
            grad_id: 0,
        }
    }
}

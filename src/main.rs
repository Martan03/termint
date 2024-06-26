use termint::{
    buffer::buffer::Buffer,
    enums::{bg::Bg, fg::Fg, modifier::Modifier, wrap::Wrap},
    geometry::{
        constrain::Constrain, coords::Coords, direction::Direction,
        rect::Rect, text_align::TextAlign,
    },
    mods,
    term::Term,
    widgets::{
        bg_grad::BgGrad,
        block::Block,
        border::{Border, BorderType},
        grad::Grad,
        layout::Layout,
        list::List,
        paragraph::Paragraph,
        span::StrSpanExtension,
        widget::Widget,
    },
};

fn main() {
    // test_block();
    // test_layout();
    // test_grad();
    // cool_example();
    // test_paragraph();
    // readme_example();
    // test_list();
    // test_layout_centering();
    // test_bg_grad();
    // term_test();
    let span = "This is a test of the span rendering".align(TextAlign::Center);
    let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 3));
    span.render(&mut buffer);

    println!("\x1b[2J");
    buffer.render();
}

#[allow(unused)]
fn test_block() {
    println!("\x1b[2J");

    let mut block = Block::new()
        .title("Not easy".to_span())
        .direction(Direction::Horizontal);

    let mut block1 = Block::new();
    let grad =
        Grad::new("This is just a basic test", (0, 220, 255), (175, 80, 255));
    block1.add_child(grad, Constrain::Percent(100));
    let block2 = Block::new().title("Test".to_span());
    let block3 =
        Block::new().title(Grad::new("Test", (100, 200, 100), (20, 160, 255)));

    block.add_child(block2, Constrain::Min(0));
    block.add_child(block1, Constrain::Min(0));
    block.add_child(block3, Constrain::Fill);

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(30, 9),
    ));
    block.render(&mut buffer);

    println!("\x1b[7B");
}

#[allow(unused)]
fn test_layout() {
    println!("\x1b[2J");
    let mut main = Block::new()
        .title("Termite".fg(Fg::Red))
        .direction(Direction::Horizontal)
        .border_type(BorderType::Double)
        .border_color(Fg::LightGray)
        .padding((0, 1));

    let mut block1 = Block::new().title("Sub block".to_span());
    let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
    block1.add_child(span1, Constrain::Percent(100));
    main.add_child(block1, Constrain::Percent(50));

    let mut block2 = Block::new().title("Another".to_span());
    let span2 =
        "This is really cool, right? This is the best place for testing"
            .fg(Fg::Blue);
    block2.add_child(span2, Constrain::Percent(100));
    main.add_child(block2, Constrain::Percent(50));

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(30, 8),
    ));
    main.render(&mut buffer);
    println!("\x1b[2B");
}

#[allow(unused)]
fn test_grad() {
    let grad = Grad::new(
        "This is a test of long text, but it is not that long",
        (0, 220, 255),
        (200, 60, 255),
    )
    .align(TextAlign::Center);
    println!("\x1b[2J");

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(10, 5),
    ));
    grad.render(&mut buffer);
}

#[allow(unused)]
fn cool_example() {
    println!("\x1b[2J");

    let mut main = Block::new()
        .title("termint".fg(Fg::Cyan))
        .border_type(BorderType::Double)
        .border_color(Fg::Gray);

    let block = Block::new()
        .title("Features:")
        .borders(Border::TOP)
        .border_color(Fg::Gray);
    main.add_child(block, Constrain::Min(0));

    let span = "Re-coloring text".fg(Fg::Red).modifiers(mods!(Italic));
    main.add_child(span, Constrain::Min(0));
    let grad = Grad::new("Gradient text", (0, 220, 255), (175, 80, 255));
    main.add_child(grad, Constrain::Min(0));

    let mut fill = Block::new()
        .title("Layout features".modifiers(mods!(Underline)))
        .border_type(BorderType::Rounded)
        .border_color(Fg::Gray)
        .direction(Direction::Horizontal);

    fill.add_child("This text fits well", Constrain::Min(0));
    let sep = Block::new().borders(Border::LEFT).border_color(Fg::Gray);
    fill.add_child(sep, Constrain::Length(1));
    fill.add_child(
        "This text will fill the rest and have ellipsis when overflows",
        Constrain::Fill,
    );

    main.add_child(fill, Constrain::Fill);

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(30, 9),
    ));
    main.render(&mut buffer);
    println!("\x1b[2B");
}

#[allow(unused)]
fn test_paragraph() {
    println!("\x1b[2J");
    let mut main = Block::new()
        .title("Paragraph")
        .direction(Direction::Horizontal);

    let mut p = Paragraph::new(vec![
        Box::new(Grad::new("this", (0, 120, 255), (120, 255, 0))),
        Box::new("This is a text in".fg(Fg::Yellow)),
        Box::new("paragraph".modifiers(vec![Modifier::Bold]).fg(Fg::Cyan)),
        Box::new("and it adds".to_span()),
        Box::new("separator".modifiers(vec![Modifier::Italic])),
        Box::new("between each span".to_span()),
    ]);

    let block = Block::new();

    main.add_child(p, Constrain::Fill);
    main.add_child(block, Constrain::Fill);

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(30, 9),
    ));
    main.render(&mut buffer);

    println!("\x1b[1B");
}

#[allow(unused)]
fn readme_example() {
    println!("\x1b[2J");

    let mut main = Block::new()
        .title("Termint".to_span())
        .direction(Direction::Horizontal)
        .border_type(BorderType::Double);

    // Creates block1 and adds span as its child
    let mut block1 = Block::new().title("Sub block".to_span());
    let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
    block1.add_child(span1, Constrain::Percent(100));
    // Adds block1 as child of main block
    main.add_child(block1, Constrain::Min(0));

    // Create block2 and adds span as its child
    let mut block2 = Block::new().title("Another".to_span());
    let span2 = "This is really cool, right?".fg(Fg::Blue);
    block2.add_child(span2, Constrain::Percent(100));
    // Adds block2 as child of main block
    main.add_child(block2, Constrain::Fill);

    // Renders the main block which renders all the children
    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(30, 9),
    ));
    main.render(&mut buffer);
    println!("\x1b[4B");
}

#[allow(unused)]
fn test_list() {
    println!("\x1b[2J");

    let mut block = Block::new();
    let list =
        List::new(vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"])
            .selected(Some(2))
            .sel_fg(Fg::Yellow)
            .sel_bg(Bg::Blue)
            .sel_char("-");
    block.add_child(list, Constrain::Fill);
    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(20, 6),
    ));
    block.render(&mut buffer);
    println!("\x1b[2B");
}

#[allow(unused)]
fn test_layout_centering() {
    println!("\x1b[2J");
    let span = Grad::new("This is a test", (0, 150, 255), (150, 255, 150))
        .wrap(Wrap::Letter);
    let mut block = Block::new().direction(Direction::Horizontal).center();
    block.add_child(span, Constrain::Min(0));

    let mut main = Block::new().direction(Direction::Vertical).center();
    main.add_child(block, Constrain::Length(4));
    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(20, 8),
    ));
    main.render(&mut buffer);
    println!("\x1b[3B");
}

#[allow(unused)]
fn test_bg_grad() {
    println!("\x1b[2J");
    let mut grad = BgGrad::new(0x0096ff, (84.71, 1.0, 0.5)).center();
    let mut layout = Layout::horizontal().center();
    layout.add_child(Block::new(), Constrain::Length(6));
    grad.add_child(layout, Constrain::Length(3));

    let mut buffer = Buffer::empty(Rect::from_coords(
        Coords::new(1, 1),
        Coords::new(20, 9),
    ));
    grad.render(&mut buffer);
    println!("\x1b[5B");
}

#[allow(unused)]
fn term_test() {
    println!("\x1b[2J");
    let small = "Too small";
    let term = Term::new().small_screen(small);

    let mut layout = Layout::vertical().padding(1);
    let mut span = "This is test of small message rendering";
    layout.add_child(span, Constrain::Length(10));

    term.render(layout);
}

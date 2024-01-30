use termint::{
    create_help,
    enums::{bg::Bg, fg::Fg, modifier::Modifier},
    geometry::{constrain::Constrain, coords::Coords, direction::Direction},
    modifiers,
    widgets::{
        block::Block,
        border::{Border, BorderType},
        grad::Grad,
        paragraph::Paragraph,
        span::StrSpanExtension,
        widget::Widget,
    },
};

fn main() {
    // test_block();
    // test_layout();
    // cool_example();
    // test_paragraph();
    readme_example();

    create_help!(
        "Test" => {"This is description"},
        "Another test" => {
            "Another description",
            "This can contain multiple literals"
        },
        "Another test" => {"Another description"}
    );
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
    block1.add_child(Box::new(grad), Constrain::Percent(100));
    let block2 = Block::new().title("Test".to_span());
    let block3 = Block::new().grad_title(Grad::new(
        "Test",
        (100, 200, 100),
        (20, 160, 255),
    ));

    block.add_child(Box::new(block2), Constrain::Fill);
    block.add_child(Box::new(block1), Constrain::Min(0));
    block.add_child(Box::new(block3), Constrain::Fill);

    block.render(&Coords::new(1, 1), &Coords::new(30, 9));

    println!("\x1b[6B");
}

#[allow(unused)]
fn test_layout() {
    println!("\x1b[2J");
    let mut main = Block::new()
        .title("Termite".fg(Fg::Red))
        .direction(Direction::Horizontal)
        .border_type(BorderType::Double)
        .border_color(Fg::LightGray);

    let mut block1 = Block::new().title("Sub block".to_span());
    let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
    block1.add_child(Box::new(span1), Constrain::Percent(100));
    main.add_child(Box::new(block1), Constrain::Percent(50));

    let mut block2 = Block::new().title("Another".to_span());
    let span2 =
        "This is really cool, right? This is the best place for testing"
            .fg(Fg::Blue);
    block2.add_child(Box::new(span2), Constrain::Percent(100));
    main.add_child(Box::new(block2), Constrain::Percent(50));

    main.render(&Coords::new(1, 1), &Coords::new(30, 8));

    println!("\x1b[1B");
}

#[allow(unused)]
fn test_grad() {
    let grad = Grad::new(
        "This is a test of long text, but it is not that long",
        (0, 220, 255),
        (200, 60, 255),
    );
    println!("\x1b[2J");
    grad.render(&Coords::new(1, 1), &Coords::new(10, 5));
}

#[allow(unused)]
fn cool_example() {
    println!("\x1b[2J");

    let mut main = Block::new()
        .title("termint".fg(Fg::Cyan))
        .border_type(BorderType::Double)
        .border_color(Fg::Gray);

    let block = Block::new()
        .title("Features:".to_span())
        .borders(Border::TOP)
        .border_color(Fg::Gray);
    main.add_child(Box::new(block), Constrain::Min(0));

    let span = "Re-coloring text".fg(Fg::Red).modifier(modifiers!(Italic));
    main.add_child(Box::new(span), Constrain::Min(0));
    let grad = Grad::new("Gradient text", (0, 220, 255), (175, 80, 255));
    main.add_child(Box::new(grad), Constrain::Min(0));

    let mut fill = Block::new()
        .title("Layout features".modifier(modifiers!(Underline)))
        .border_type(BorderType::Rounded)
        .border_color(Fg::Gray)
        .direction(Direction::Horizontal);

    let long = "This text fits well".to_span();
    fill.add_child(Box::new(long), Constrain::Min(0));
    let sep = Block::new().borders(Border::LEFT).border_color(Fg::Gray);
    fill.add_child(Box::new(sep), Constrain::Length(1));
    let fill_text =
        "This text will fill the rest and have ellipsis when overflows"
            .to_span();
    fill.add_child(Box::new(fill_text), Constrain::Fill);

    main.add_child(Box::new(fill), Constrain::Fill);

    main.render(&Coords::new(1, 1), &Coords::new(40, 9));
    println!("\x1b[1B");
}

#[allow(unused)]
fn test_paragraph() {
    println!("\x1b[2J");

    let mut main = Block::new()
        .title("Paragraph".to_span())
        .direction(Direction::Horizontal);

    let mut p = Paragraph::new(vec![
        "This is a text in".fg(Fg::Yellow),
        "paragraph".modifier(vec![Modifier::Bold]).fg(Fg::Cyan),
        "and it adds".to_span(),
        "separator".modifier(vec![Modifier::Italic]),
        "between each span".to_span(),
    ]);

    let block = Block::new();

    main.add_child(Box::new(p), Constrain::Min(0));
    main.add_child(Box::new(block), Constrain::Fill);

    main.render(&Coords::new(1, 1), &Coords::new(20, 9));
    println!("\x1b[6B");
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
    block1.add_child(Box::new(span1), Constrain::Percent(100));
    // Adds block1 as child of main block
    main.add_child(Box::new(block1), Constrain::Min(0));

    // Create block2 and adds span as its child
    let mut block2 = Block::new().title("Another".to_span());
    let span2 = "This is really cool, right?".fg(Fg::Blue);
    block2.add_child(Box::new(span2), Constrain::Percent(100));
    // Adds block2 as child of main block
    main.add_child(Box::new(block2), Constrain::Fill);

    // Renders the main block which renders all the children
    main.render(&Coords::new(1, 1), &Coords::new(30, 8));
    println!("\x1b[3B");
}

use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::{constrain::Constrain, coords::Coords, direction::Direction},
    widgets::{
        block::Block, border::BorderType, grad::Grad, span::StrSpanExtension,
        widget::Widget,
    },
};

fn main() {
    let grad = Grad::new(
        "This is a test of long text, but it is not that long so I have to add",
        (0, 220, 255),
        (200, 60, 255),
    );
    println!("\x1b[2J");
    grad.render(&Coords::new(1, 1), &Coords::new(10, 5));
    // test_block();
    // test_layout();
}

#[allow(unused)]
fn test_block() {
    println!("\x1b[2J");

    let block = Block::new().title("Not easy".to_span());
    block.render(&Coords::new(1, 1), &Coords::new(20, 1));

    println!("\x1b[4B");
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

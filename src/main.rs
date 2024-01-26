use termite::{
    enums::{bg::Bg, fg::Fg},
    geometry::{constrain::Constrain, coords::Coords, direction::Direction},
    widgets::{
        block::Block, border::BorderType, span::StrSpanExtension,
        widget::Widget,
    },
};

fn main() {
    // test_block();
    test_layout();
}

#[allow(unused)]
fn test_block() {
    println!("\x1b[2J");

    let block = Block::new().title("Not easy");
    block.render(&Coords::new(1, 1), &Coords::new(20, 1));

    println!("\x1b[4B");
}

fn test_layout() {
    println!("\x1b[2J");
    let mut block = Block::new()
        .title("This is cool")
        .direction(Direction::Horizontal)
        .border_type(BorderType::Double);

    let block1 = Block::new().title("Sub block");
    block.add_child(Box::new(block1), Constrain::Percent(50));

    let mut block2 = Block::new().title("Another");
    let span = "This is really cool, right?".fg(Fg::Blue);
    let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
    block2.add_child(Box::new(span), Constrain::Length(3));
    block2.add_child(Box::new(span1), Constrain::Length(1));

    block.add_child(Box::new(block2), Constrain::Percent(50));

    block.render(&Coords::new(1, 1), &Coords::new(31, 8));

    println!("\x1b[1B");
}

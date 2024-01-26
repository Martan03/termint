use termite::{
    enums::fg::Fg, geometry::{constrain::Constrain, coords::Coords, direction::Direction}, widgets::{block::Block, span::StrSpanExtension, widget::Widget}
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
        .direction(Direction::Horizontal);

    let block1 = Block::new().title("Sub block");
    block.add_child(Box::new(block1), Constrain::Percent(50));

    let mut block2 = Block::new().title("Another");
    let span = "This is really cool, right?".fg(Fg::Blue);
    block2.add_child(Box::new(span), Constrain::Percent(100));

    block.add_child(Box::new(block2), Constrain::Percent(50));

    block.render(&Coords::new(1, 1), &Coords::new(31, 8));

    println!("\x1b[2B");
}

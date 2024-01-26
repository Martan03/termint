use termite::{
    geometry::{constrain::Constrain, coords::Coords},
    widgets::{block::Block, layout::Layout, widget::Widget},
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
    let mut layout = Layout::horizontal();

    let block = Block::new().title("Title");
    layout.child(Box::new(block), Constrain::Percent(80));

    let block2 = Block::new().title("Block");
    layout.child(Box::new(block2), Constrain::Length(10));

    let block3 = Block::new().title("Ending");
    layout.child(Box::new(block3), Constrain::Length(10));

    layout.render(&Coords::new(1, 1), &Coords::new(30, 7));
}

use termite::{
    geometry::{constrain::Constrain, coords::Coords},
    widgets::{block::Block, layout::Layout, widget::Widget},
};

fn main() {
    test_layout();
}

fn test_layout() {
    println!("\x1b[2J");
    let mut layout = Layout::vertical();

    let block = Block::new().title("Block");
    layout.child(Box::new(block), Constrain::Length(3));

    let block2 = Block::new().title("Block 2");
    layout.child(Box::new(block2), Constrain::Length(5));

    layout.render(&Coords::new(0, 0), &Coords::new(30, 20));

    println!("\x1b[4B");
}

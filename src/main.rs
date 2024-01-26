use termite::{geometry::coords::Coords, widgets::{block::Block, widget::Widget}};

fn main() {
    test_block();
}

fn test_block() {
    println!("\x1b[2J");
    let block = Block::new().title("Block");
    block.render(Coords::new(0, 0), Coords::new(15, 5));

    println!("\x1b[3B");
}

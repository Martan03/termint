use termite::{geometry::coords::Coords, widgets::block::Block};

fn main() {
    test_block();
}

fn test_block() {
    println!("\x1b[2J");
    let block = Block::new().title("Block");
    block.render(Coords::new(15, 5), Coords::new(0, 0));

    println!("\x1b[3B");
}

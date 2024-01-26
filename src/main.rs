use std::sync::Condvar;

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
    let mut block = Block::new().title("This is cool");

    let block1 = Block::new().title("Sub block");
    block.add_child(Box::new(block1), Constrain::Percent(50));

    let block2 = Block::new().title("Another");
    block.add_child(Box::new(block2), Constrain::Percent(50));

    block.render(&Coords::new(1, 1), &Coords::new(21, 10));

    println!("\x1b[4B");
}

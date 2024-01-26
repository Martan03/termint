extern crate termite;

#[cfg(test)]
mod tests {
    use termite::{geometry::coords::Coords, widgets::{block::Block, widget::Widget}};

    #[test]
    fn block_render() {
        let block = Block::new().title("Block");
        block.render(Coords::new(0, 0), Coords::new(15, 5));
    }
}

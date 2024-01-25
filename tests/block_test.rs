extern crate termite;

#[cfg(test)]
mod tests {
    use termite::{geometry::coords::Coords, widgets::block::Block};

    #[test]
    fn block_render() {
        let block = Block::new().title("Block");
        block.render(Coords::new(15, 5), Coords::new(0, 0));
    }
}

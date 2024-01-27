extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{
        geometry::coords::Coords,
        widgets::{block::Block, span::StrSpanExtension, widget::Widget},
    };

    #[test]
    fn block_render() {
        let block = Block::new().title("Block".to_span());
        block.render(&Coords::new(0, 0), &Coords::new(15, 5));
    }
}

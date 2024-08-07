extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{
        buffer::Buffer,
        geometry::{Coords, Rect},
        widgets::{Block, StrSpanExtension, Widget},
    };

    #[test]
    fn block_render() {
        let block = Block::vertical().title("Block".to_span());
        let mut buffer = Buffer::empty(Rect::from_coords(
            Coords::new(1, 1),
            Coords::new(15, 5),
        ));
        block.render(&mut buffer);
    }
}

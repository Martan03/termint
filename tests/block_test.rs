extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{
        buffer::Buffer,
        geometry::{Vec2, Rect},
        widgets::{Block, StrSpanExtension, Widget},
    };

    #[test]
    fn block_render() {
        let block = Block::vertical().title("Block".to_span());
        let mut buffer = Buffer::empty(Rect::from_coords(
            Vec2::new(1, 1),
            Vec2::new(15, 5),
        ));
        block.render(&mut buffer);
    }
}

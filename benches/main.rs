use criterion::criterion_main;

pub mod buffer;
pub mod render;
pub mod span;

criterion_main!(
    buffer::benches,
    span::benches,
    render::layout::benches,
    render::grid::benches,
    render::scrollbar::benches,
);

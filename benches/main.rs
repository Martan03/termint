use criterion::criterion_main;

pub mod buffer;
pub mod span;

criterion_main!(buffer::benches, span::benches);

use criterion::criterion_main;

pub mod buffer;

criterion_main!(buffer::benches);

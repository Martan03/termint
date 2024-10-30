use criterion::{black_box, criterion_group, criterion_main, Criterion};
use termint::{buffer::Buffer, geometry::Rect};

fn benchmark_merge(c: &mut Criterion) {
    let mut buffer = Buffer::empty(Rect::new(1, 1, 255, 255));
    let sbuffer = Buffer::empty(Rect::new(1, 1, 127, 127));

    c.bench_function("merge_function", |b| {
        b.iter(|| buffer.merge(black_box(sbuffer.clone())));
    });
}

criterion_group!(benches, benchmark_merge);
criterion_main!(benches);

use criterion::{black_box, criterion_group, Criterion};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Unit},
    widgets::{Grid, Spacer, Widget},
};

fn benchmark_child_rendering(c: &mut Criterion) {
    let mut grid =
        Grid::new(vec![Unit::Length(1); 100], vec![Unit::Length(1); 100]);
    for y in 0..100 {
        for x in 0..100 {
            grid.push(Spacer::new(), x, y);
        }
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);
    c.bench_function("child_rendering", |b| {
        b.iter(|| {
            grid.render(black_box(&mut buffer.clone()), black_box(rect))
            // grid.render(black_box(&mut buffer.clone()))
        });
    });
}

criterion_group!(benches, benchmark_child_rendering);

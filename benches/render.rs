use criterion::{black_box, criterion_group, Criterion};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Unit},
    widgets::{cache::Cache, Element, Grid, Layout, Spacer, Widget},
};

fn child_render(c: &mut Criterion) {
    let mut grid =
        Grid::new(vec![Unit::Length(1); 100], vec![Unit::Length(1); 100]);
    for y in 0..100 {
        for x in 0..100 {
            grid.push(Spacer::new(), x, y);
        }
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);
    let grid: Element = grid.into();
    c.bench_function("child_render", |b| {
        b.iter(|| {
            let mut cache = Cache::new();
            cache.diff(&grid);
            grid.render(
                black_box(&mut buffer.clone()),
                black_box(rect),
                black_box(&mut cache),
            );
        });
    });
}

fn cache_render(c: &mut Criterion) {
    let mut layout = Layout::horizontal();
    for i in 0..100 {
        layout.push("word ".repeat((i % 15) + 1), 0..);
    }

    let rect = Rect::new(1, 1, 1000, 10);
    let buffer = Buffer::empty(rect);
    let mut cache = Cache::new();

    let layout: Element = layout.into();
    cache.diff(&layout);
    layout.render(&mut buffer.clone(), rect, &mut cache);

    c.bench_function("cache_render", |b| {
        b.iter(|| {
            cache.diff(&layout);
            layout.render(
                black_box(&mut buffer.clone()),
                black_box(rect),
                black_box(&mut cache),
            );
        });
    });
}

fn no_cache_render(c: &mut Criterion) {
    let mut layout = Layout::horizontal();
    for i in 0..100 {
        layout.push("word ".repeat((i % 15) + 1), 0..);
    }

    let rect = Rect::new(1, 1, 1000, 10);
    let buffer = Buffer::empty(rect);

    let layout: Element = layout.into();
    c.bench_function("no_cache_render", |b| {
        b.iter(|| {
            let mut cache = Cache::new();
            cache.diff(&layout);

            layout.render(
                black_box(&mut buffer.clone()),
                black_box(rect),
                black_box(&mut cache),
            );
        });
    });
}

criterion_group!(benches, child_render, cache_render, no_cache_render);

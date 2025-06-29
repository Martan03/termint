use criterion::{black_box, criterion_group, Criterion};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Unit},
    widgets::{cache::Cache, Element, Grid, Spacer, Widget},
};

fn grid_cache_render(c: &mut Criterion) {
    let mut grid = Grid::empty();
    let col_options = [Unit::Percent(1), Unit::Fill(1), Unit::Length(1)];
    let row_options = [Unit::Fill(1), Unit::Length(1)];
    for i in 0..100 {
        grid.col(col_options[i % 3]);
        grid.row(row_options[i % 2]);
    }
    for y in 0..100 {
        for x in 0..100 {
            grid.push(Spacer::new(), x, y);
        }
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);
    let mut cache = Cache::new();

    let grid: Element = grid.into();
    cache.diff(&grid);
    grid.render(&mut buffer.clone(), rect, &mut cache);

    c.bench_function("grid_cache_render", |b| {
        b.iter(|| {
            cache.diff(&grid);
            grid.render(
                black_box(&mut buffer.clone()),
                black_box(rect),
                black_box(&mut cache),
            );
        });
    });
}

fn grid_no_cache_render(c: &mut Criterion) {
    let mut grid = Grid::empty();
    let col_options = [Unit::Percent(1), Unit::Fill(1), Unit::Length(1)];
    let row_options = [Unit::Fill(1), Unit::Length(1)];
    for i in 0..100 {
        grid.col(col_options[i % 3]);
        grid.row(row_options[i % 2]);
    }
    for y in 0..100 {
        for x in 0..100 {
            grid.push(Spacer::new(), x, y);
        }
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);

    let grid: Element = grid.into();
    c.bench_function("grid_no_cache_render", |b| {
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

criterion_group!(benches, grid_cache_render, grid_no_cache_render);

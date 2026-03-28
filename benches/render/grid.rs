use criterion::{Criterion, black_box, criterion_group};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Unit},
    widgets::{Element, Grid, LayoutNode, Spacer, Widget},
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

    let grid: Element = grid.into();
    let mut layout = LayoutNode::new(&grid);
    grid.render(&mut buffer.clone(), &layout);

    c.bench_function("grid_cache_render", |b| {
        b.iter(|| {
            layout.diff(&grid, &grid);
            grid.render(black_box(&mut buffer.clone()), black_box(&layout));
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
            let layout = LayoutNode::new(&grid);
            grid.render(black_box(&mut buffer.clone()), black_box(&layout));
        });
    });
}

criterion_group!(benches, grid_cache_render, grid_no_cache_render);

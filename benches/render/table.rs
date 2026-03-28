use std::{cell::RefCell, rc::Rc};

use criterion::{Criterion, black_box, criterion_group};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Unit},
    widgets::{
        Element, LayoutNode, Row, Table, TableState, Widget, cache::Cache,
    },
};

fn table_cache_render(c: &mut Criterion) {
    let state = Rc::new(RefCell::new(
        TableState::new(0).selected(0).selected_column(0),
    ));
    let table = Table::new(
        get_data(),
        vec![Unit::Length(4), Unit::Percent(40), Unit::Fill(1)],
        state,
    );

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);
    let mut cache = Cache::new();

    let grid: Element = table.into();
    let mut node = LayoutNode::new(&grid);
    cache.diff(&grid);
    grid.render(&mut buffer.clone(), &node, &mut cache);

    c.bench_function("table_cache_render", |b| {
        b.iter(|| {
            node.diff(&grid, &grid);
            cache.diff(&grid);
            grid.render(
                black_box(&mut buffer.clone()),
                black_box(&node),
                black_box(&mut cache),
            );
        });
    });
}

fn table_no_cache_render(c: &mut Criterion) {
    let state = Rc::new(RefCell::new(
        TableState::new(0).selected(0).selected_column(0),
    ));
    let table = Table::new(
        get_data(),
        vec![Unit::Length(4), Unit::Percent(40), Unit::Fill(1)],
        state,
    );

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);

    let grid: Element = table.into();
    c.bench_function("table_no_cache_render", |b| {
        b.iter(|| {
            let node = LayoutNode::new(&grid);
            let mut cache = Cache::new();
            cache.diff(&grid);
            grid.render(
                black_box(&mut buffer.clone()),
                black_box(&node),
                black_box(&mut cache),
            );
        });
    });
}

fn get_data() -> Vec<Row<()>> {
    let mut rows = vec![];
    for i in 0..100 {
        rows.push(Row::new(vec![
            format!("{}", i),
            format!("Value {}", i * 2),
            format!("Description {}", i * 3),
        ]));
    }
    rows
}

criterion_group!(benches, table_cache_render, table_no_cache_render);

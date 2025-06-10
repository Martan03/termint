use std::{cell::Cell, rc::Rc};

use criterion::{black_box, criterion_group, Criterion};
use termint::{
    buffer::Buffer,
    geometry::{Constraint, Rect},
    widgets::{
        cache::Cache, Element, Layout, Scrollbar, ScrollbarState, Widget,
    },
};

fn scrollbar_cache_render(c: &mut Criterion) {
    let mut vlayout = Layout::horizontal();
    let mut hlayout = Layout::vertical();
    for i in 0..50 {
        let state = Rc::new(Cell::new(ScrollbarState::new(i).content_len(50)));
        vlayout.push(Scrollbar::vertical(state.clone()), 1);
        hlayout.push(Scrollbar::horizontal(state.clone()), 1);
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);
    let mut cache = Cache::new();

    let mut layout = Layout::horizontal();
    layout.push(vlayout, Constraint::Fill(1));
    layout.push(hlayout, Constraint::Fill(1));

    let layout: Element = layout.into();
    cache.diff(&layout);
    layout.render(&mut buffer.clone(), rect, &mut cache);

    c.bench_function("scrollbar_cache_render", |b| {
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

fn scrollbar_no_cache_render(c: &mut Criterion) {
    let mut vlayout = Layout::horizontal();
    let mut hlayout = Layout::vertical();
    for i in 0..50 {
        let state = Rc::new(Cell::new(ScrollbarState::new(i).content_len(50)));
        vlayout.push(Scrollbar::vertical(state.clone()), 1);
        hlayout.push(Scrollbar::horizontal(state.clone()), 1);
    }

    let rect = Rect::new(1, 1, 101, 101);
    let buffer = Buffer::empty(rect);

    let mut layout = Layout::horizontal();
    layout.push(vlayout, Constraint::Fill(1));
    layout.push(hlayout, Constraint::Fill(1));

    let layout: Element = layout.into();
    c.bench_function("scrollbar_no_cache_render", |b| {
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

criterion_group!(benches, scrollbar_cache_render, scrollbar_no_cache_render);

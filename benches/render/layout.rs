use criterion::{Criterion, black_box, criterion_group};
use termint::{
    buffer::Buffer,
    geometry::Rect,
    widgets::{Element, Layout, LayoutNode, Widget, cache::Cache},
};

fn layout_cache_render(c: &mut Criterion) {
    let mut layout = Layout::horizontal();
    for i in 0..100 {
        layout.push("word ".repeat((i % 15) + 1), 0..);
    }

    let rect = Rect::new(1, 1, 1000, 10);
    let buffer = Buffer::empty(rect);
    let mut cache = Cache::new();

    let layout: Element = layout.into();
    let mut node = LayoutNode::new(&layout);
    cache.diff(&layout);
    layout.render(&mut buffer.clone(), &node, &mut cache);

    c.bench_function("layout_cache_render", |b| {
        b.iter(|| {
            node.diff(&layout, &layout);
            cache.diff(&layout);
            layout.render(
                black_box(&mut buffer.clone()),
                black_box(&node),
                black_box(&mut cache),
            );
        });
    });
}

fn layout_no_cache_render(c: &mut Criterion) {
    let mut layout = Layout::horizontal();
    for i in 0..100 {
        layout.push("word ".repeat((i % 15) + 1), 0..);
    }

    let rect = Rect::new(1, 1, 1000, 10);
    let buffer = Buffer::empty(rect);

    let layout: Element = layout.into();
    c.bench_function("layout_no_cache_render", |b| {
        b.iter(|| {
            let node = LayoutNode::new(&layout);
            let mut cache = Cache::new();
            cache.diff(&layout);

            layout.render(
                black_box(&mut buffer.clone()),
                black_box(&node),
                black_box(&mut cache),
            );
        });
    });
}

criterion_group!(benches, layout_cache_render, layout_no_cache_render);

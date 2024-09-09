use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use termint::{
    buffer::Buffer,
    enums::{Color, Modifier, Wrap},
    geometry::{Constraint, Rect, TextAlign, Unit, Vec2},
    style::Style,
    term::Term,
    widgets::{
        BgGrad, Block, Border, BorderType, Grad, Grid, Layout, List,
        ListState, Paragraph, Scrollable, Scrollbar, ScrollbarState, Spacer,
        StrSpanExtension, Widget,
    },
};

fn main() {
    // test_block();
    // test_layout();
    // test_grad();
    cool_example();
    // test_paragraph();
    // readme_example();
    // test_list();
    // test_layout_centering();
    // test_bg_grad();
    // term_test();
    // grid_test();
    // diff_render_test();
    // merge_test();
    // scrollbar_test();
    // scrollable_test();
}

#[allow(unused)]
fn test_block() {
    let mut block = Block::horizontal().title("Not easy");

    let block1 = Block::vertical().title("Test");
    let mut block2 = Block::vertical();
    let grad =
        Grad::new("This is just a basic test", (0, 220, 255), (175, 80, 255));
    block2.push(grad, Constraint::Percent(100));
    let block3 = Block::vertical().title(Grad::new(
        "Test",
        (100, 200, 100),
        (20, 160, 255),
    ));

    block.push(block1, Constraint::Min(0));
    block.push(block2, Constraint::Min(0));
    block.push(block3, Constraint::Fill(1));

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 9)));
    block.render(&mut buffer);
    buffer.render();

    // println!("\x1b[7B");
}

#[allow(unused)]
fn test_layout() {
    println!("\x1b[2J");
    let mut main = Block::horizontal()
        .title("Termite".fg(Color::Red))
        .border_type(BorderType::Double)
        .border_color(Color::LightGray)
        .padding((0, 1));

    let mut block1 = Block::vertical().title("Sub block");
    let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
    block1.push(span1, Constraint::Percent(100));
    main.push(block1, Constraint::Percent(50));

    let mut block2 = Block::vertical().title("Another");
    let span2 =
        "This is really cool, right? This is the best place for testing"
            .fg(Color::Blue);
    block2.push(span2, Constraint::Percent(100));
    main.push(block2, Constraint::Percent(50));

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 8)));
    main.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn test_grad() {
    let grad = Grad::new(
        "This is a test of long text, but it is not that long",
        (0, 220, 255),
        (200, 60, 255),
    )
    .wrap(Wrap::Letter)
    .align(TextAlign::Center);
    println!("\x1b[2J");

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(10, 5)));
    grad.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn cool_example() {
    println!("\x1b[2J");

    let mut main = Block::vertical()
        .title("termint".fg(Color::Cyan))
        .border_type(BorderType::Double)
        .border_color(Color::Gray);

    let block = Block::vertical()
        .title("Features:")
        .borders(Border::TOP)
        .border_color(Color::Gray);
    main.push(block, Constraint::Min(0));

    let span = "Re-coloring".fg(Color::Red).modifier(Modifier::ITALIC);
    main.push(span, Constraint::Min(0));
    let grad = Grad::new("Gradient text", (0, 220, 255), (175, 80, 255));
    main.push(grad, Constraint::Min(0));

    let mut fill = Block::horizontal()
        .title("Layout features".modifier(Modifier::UNDERLINED))
        .border_type(BorderType::Rounded)
        .border_color(Color::Gray);

    fill.push("This text fits well", Constraint::Min(0));
    let sep = Block::vertical()
        .borders(Border::LEFT)
        .border_color(Color::Gray);
    fill.push(sep, Constraint::Length(1));
    fill.push(
        "This text will fill the rest and have ellipsis when overflows",
        Constraint::Fill(1),
    );

    main.push(fill, Constraint::Fill(1));

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 9)));
    main.render(&mut buffer);

    buffer.render();
}

#[allow(unused)]
fn test_paragraph() {
    println!("\x1b[2J");
    let mut main = Block::horizontal().title("Paragraph");

    let mut p = Paragraph::new(vec![
        "This is a text in".fg(Color::Yellow).into(),
        Grad::new("this", (0, 120, 255), (120, 255, 0)).into(),
        "paragraph".modifier(Modifier::BOLD).fg(Color::Cyan).into(),
        "and it adds".to_span().into(),
        "separator".modifier(Modifier::ITALIC).into(),
        "between each span".to_span().into(),
    ]);

    let block = Block::vertical();

    main.push(p, Constraint::Fill(1));
    main.push(block, Constraint::Fill(1));

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 8)));
    main.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn readme_example() {
    println!("\x1b[2J");

    let mut main = Block::horizontal()
        .title("Termint".to_span())
        .border_type(BorderType::Double);

    // Creates block1 and adds span as its child
    let mut block1 = Block::vertical().title("Sub block");
    let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
    block1.push(span1, Constraint::Percent(100));
    // Adds block1 as child of main block
    main.push(block1, Constraint::Min(0));

    // Create block2 and adds span as its child
    let mut block2 = Block::horizontal().title("Another");
    let span2 = "This is really cool, right?".fg(Color::Blue);
    block2.push(span2, Constraint::Percent(100));
    // Adds block2 as child of main block
    main.push(block2, Constraint::Fill(1));

    // Renders the main block which renders all the children
    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 9)));
    main.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn test_list() {
    println!("\x1b[2J");

    let mut offset = Rc::new(RefCell::new(ListState::selected(0, 2)));

    let mut block = Block::vertical();
    let list = List::new(
        vec!["Item1", "Item2", "Item3", "Item4", "Item5", "Item6"],
        offset.clone(),
    )
    .selected_style(Style::new().fg(Color::Yellow).bg(Color::Blue))
    .highlight_symbol(">")
    .highlight_style(Style::new().fg(Color::Red).modifier(Modifier::BOLD));

    block.push(list, Constraint::Fill(1));
    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(20, 6)));
    // block.render(&mut buffer);
    // buffer.render();

    let mut term = Term::new();
    term.render(block);

    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(20, 6)));
    offset.borrow_mut().selected = Some(4);
    // block.render(&mut buffer);

    // buffer.render();
    term.rerender();
}

#[allow(unused)]
fn test_layout_centering() {
    println!("\x1b[2J");
    let span = Grad::new("This is a test", (0, 150, 255), (150, 255, 150))
        .wrap(Wrap::Letter);
    let mut block = Block::horizontal().center();
    block.push(span, Constraint::Min(0));

    let mut main = Block::vertical().center();
    main.push(block, Constraint::Length(4));
    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(20, 8)));
    main.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn test_bg_grad() {
    println!("\x1b[2J");
    // let mut grad = BgGrad::horizontal(0x0096ff, (84.71, 1.0, 0.5)).center();
    // let mut layout = Layout::horizontal().center();
    // layout.push(Block::vertical(), Constraint::Length(6));
    // grad.push(layout, Constraint::Length(3));

    // let mut buffer =
    //     Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(20, 9)));
    // grad.render(&mut buffer);
    // buffer.render();
}

#[allow(unused)]
fn term_test() {
    println!("\x1b[2J");
    let small = "Too small";
    let mut term = Term::new().small_screen(small);

    let mut layout = Layout::vertical().padding(1);
    let mut span = "This is test of small message rendering";
    layout.push(span, Constraint::Length(9));

    term.render(layout);
}

#[allow(unused)]
fn grid_test() {
    let mut grid = Grid::new(
        vec![Unit::Length(3), Unit::Length(4), Unit::Fill(1)],
        vec![Unit::Fill(1), Unit::Length(1), Unit::Fill(1)],
    );

    grid.push(Block::vertical(), 1, 0);
    grid.push(Block::vertical(), 0, 1);
    grid.push(Block::vertical(), 1, 2);
    grid.push(Block::vertical(), 2, 1);
    grid.push("Grid", 1, 1);

    let mut buffer = Buffer::empty(Rect::new(1, 1, 15, 6));
    grid.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn diff_render_test() {
    let mut main = Block::horizontal()
        .title("Termint".to_span())
        .border_type(BorderType::Double);

    // Creates block1 and adds span as its child
    let mut block1 = Block::vertical().title("Sub block");
    let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
    block1.push(span1, Constraint::Percent(100));
    // Adds block1 as child of main block
    main.push(block1, Constraint::Min(0));

    // Create block2 and adds span as its child
    let mut block2 = Block::horizontal().title("Another");
    let span2 = "This is really cool".fg(Color::Blue);
    block2.push(span2, Constraint::Percent(100));
    // Adds block2 as child of main block
    main.push(block2, Constraint::Fill(1));

    // Renders the main block which renders all the children
    let mut dbuffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 9)));
    main.render(&mut dbuffer);
    // dbuffer.render();

    let mut main = Block::horizontal()
        .title("Termint".to_span())
        .border_type(BorderType::Double);

    // Creates block1 and adds span as its child
    let mut block1 = Block::vertical().title("Sub block");
    let span1 = "I like it!".fg(Color::Green).bg(Color::Yellow);
    block1.push(span1, Constraint::Percent(100));
    // Adds block1 as child of main block
    main.push(block1, Constraint::Min(0));

    // Create block2 and adds span as its child
    let mut block2 = Block::horizontal().title("Another");
    let span2 = "This is really cool, right?".fg(Color::Blue);
    block2.push(span2, Constraint::Percent(100));
    // Adds block2 as child of main block
    main.push(block2, Constraint::Fill(1));

    // Renders the main block which renders all the children
    let mut buffer =
        Buffer::empty(Rect::from_coords(Vec2::new(1, 1), Vec2::new(30, 9)));
    main.render(&mut buffer);
    buffer.render_diff(&dbuffer);
}

#[allow(unused)]
fn merge_test() {
    println!("\x1b[2J");

    let mut block1 = Block::vertical();
    block1.push("This will be covered", Constraint::Min(0));
    let mut buffer = Buffer::empty(Rect::new(1, 1, 8, 5));
    block1.render(&mut buffer);

    let mut block2 = Block::vertical();
    block2.push("This will go above", Constraint::Min(0));
    let mut sbuffer = Buffer::empty(Rect::new(4, 3, 7, 5));
    block2.render(&mut sbuffer);

    buffer.merge(sbuffer);
    buffer.render();
}

#[allow(unused)]
fn scrollbar_test() {
    println!("\x1b[2J");

    let state = Rc::new(Cell::new(ScrollbarState::new(3).content_len(30)));

    let vertical = Scrollbar::vertical(state.clone());
    let horizontal = Scrollbar::horizontal(state.clone());

    let mut grid = Grid::new(
        vec![Unit::Fill(1), Unit::Length(1)],
        vec![Unit::Fill(1), Unit::Length(1)],
    );
    grid.push(vertical, 1, 0);
    grid.push(horizontal, 0, 1);

    let mut buffer = Buffer::empty(Rect::new(1, 1, 12, 7));
    grid.render(&mut buffer);
    buffer.render();
}

#[allow(unused)]
fn scrollable_test() {
    println!("\x1b[2J");

    // // Widget to wrap scrollable around
    // let span = "Long text that cannot fit so scrolling is needed".to_span();
    // // Scrollable state containing offset
    // let state = Rc::new(Cell::new(ScrollbarState::new(2)));
    // // Creates scrollable widget with vertical scrolling
    // let scrollable = Scrollable::vertical(span, state);
    // // Renders using the buffer
    // let mut buffer = Buffer::empty(Rect::new(1, 1, 9, 5));
    // scrollable.render(&mut buffer);
    // buffer.render();

    // let mut layout = Layout::horizontal();
    // layout.push("Test", Constraint::Length(20));

    // let mut bg = BgGrad::vertical((10, 250, 30), (200, 60, 120));
    // bg.push(layout, Constraint::Length(10));

    // let vstate = Rc::new(Cell::new(ScrollbarState::new(3)));
    // let hstate = Rc::new(Cell::new(ScrollbarState::new(1)));
    // let scrollable = Scrollable::both(bg, vstate.clone(), hstate.clone());

    // // let vstate = Rc::new(Cell::new(ScrollbarState::new(2)));
    // // let scrollable = Scrollable::horizontal(
    // //     "This is a test of new widget with very long text".to_span(),
    // //     vstate.clone(),
    // // );

    // let mut buffer = Buffer::empty(Rect::new(1, 1, 10, 5));
    // scrollable.render(&mut buffer);
    // buffer.render();
}

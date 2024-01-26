# termite

Rust library for colored printing and Terminal User Interfaces

## Installation:

This library will be available on crates.io (not yet)

## Basic example:

Printing colored text is really easy, you can do it like this:

```rust
println!("{}", "Cyan text".fg(Fg::Cyan));

println!("{}", "Cyan text on white background".fg(Fg::Cyan).bg(Bg::White));

println!("{}", "Bold red text".fg(Fg::Red).modifiers(vec![Modifier::Bold]));

println!("{}", "Text with RGB value".fg(Fg::RGB(2, 249, 171)));
```

You can see all the colors and modifiers in the documentation.

## Advanced example:

You can also create TUIs using this library:

```rust
// Creates main block and sets its properties
let mut main = Block::new()
    .title("Termite")
    .direction(Direction::Horizontal)
    .border_type(BorderType::Double);

/// Creates block1 and adds span as its child
let mut block1 = Block::new().title("Sub block");
let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
block1.add_child(Box::new(span1), Constrain::Percent(100));
/// Adds block1 as child of main block
main.add_child(Box::new(block1), Constrain::Percent(50));

/// Create block2 and adds span as its child
let mut block2 = Block::new().title("Another");
let span2 = "This is really cool, right?".fg(Fg::Blue);
block2.add_child(Box::new(span2), Constrain::Percent(100));
/// Adds block2 as child of main block
main.add_child(Box::new(block2), Constrain::Percent(50));

/// Renders the main block which renders all the children
main.render(&Coords::new(1, 1), &Coords::new(30, 8));
```

## Usage:

Code blocks above are just examples of the usage. To see more about functions,
Widgets and more, please visit the documentation.

## Technologies

Obviously this library was created in Rust, but I also used library called
[term-size](https://docs.rs/term_size/latest/term_size/) to get terminal size.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termite](https://github.com/Martan03/termite)
- **Author website:** [martan03.github.io](https://martan03.github.io)

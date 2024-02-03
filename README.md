# termint

![Crates.io Version](https://img.shields.io/crates/v/termint?logo=rust)
![docs.rs](https://img.shields.io/docsrs/termint?logo=rust)
![Crates.io Total Downloads](https://img.shields.io/crates/d/termint)

Rust library for colored printing and Terminal User Interfaces

## Installation:

This library is available on [crates.io](https://crates.io/crates/termint).
You can add it to your projects using cargo:
```terminal
cargo add termint
```

## Basic example:

Printing colored text is really easy, you can do it like this:

```rust
println!("{}", "Cyan text".fg(Fg::Cyan));
println!("{}", "Cyan text on white background".fg(Fg::Cyan).bg(Bg::White));
println!("{}", "Bold red text".fg(Fg::Red).modifier(vec![Modifier::Bold]));
println!("{}", "Text with RGB value".fg(Fg::RGB(0, 249, 210)));
```
![image](https://github.com/Martan03/termint/assets/46300167/c906a565-69b5-4664-9db0-ad89ff457cbb)

You can see all the colors and modifiers in the
[documentation](https://docs.rs/termint/latest/termint/).

## Advanced example:

You can also create TUIs using this library. This example shows how you can
use Block widget and add children to it and creating Layout:

```rust
// Creates main block and sets its properties
let mut main = Block::new()
    .title("Termint".to_span())
    .direction(Direction::Horizontal)
    .border_type(BorderType::Double);

// Creates block1 and adds span as its child
let mut block1 = Block::new().title("Sub block".to_span());
let span1 = "I like it!".fg(Fg::Green).bg(Bg::Yellow);
block1.add_child(span1, Constrain::Percent(100));
// Adds block1 as child of main block
main.add_child(block1, Constrain::Min(0));

// Create block2 and adds span as its child
let mut block2 = Block::new().title("Another".to_span());
let span2 = "This is really cool, right?".fg(Fg::Blue);
block2.add_child(span2, Constrain::Percent(100));
// Adds block2 as child of main block
main.add_child(block2, Constrain::Fill);

// Renders the main block which renders all the children
main.render(&Coords::new(1, 1), &Coords::new(30, 8));
```
![image](https://github.com/Martan03/termint/assets/46300167/cdd0850b-1952-4c4b-8dec-b49c30d59f6d)

## Usage:

Code blocks above are just examples of the usage. To see more about functions,
Widgets and more, please visit the
[documentation](https://docs.rs/termint/latest/termint/).

## Technologies

Obviously this library was created in Rust, but I also used library called
[term_size](https://docs.rs/term_size/latest/term_size/) to get terminal size.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termint](https://github.com/Martan03/termint)
- **Package**: [crates.io](https://crates.io/crates/termint)
- **Documentation**: [docs.rs](https://docs.rs/termint/latest/termint/)
- **Author website:** [martan03.github.io](https://martan03.github.io)

/// Macro to combine [`Border`](crate::enums::Border) sides
///
/// ## Usage:
/// ```rust
/// # use termint::{borders, enums::Border};
/// // Without macro:
/// let top_left_right = Border::TOP | Border::LEFT | Border::RIGHT;
/// // With macro:
/// let top_left_right = borders!(TOP, LEFT, RIGHT);
/// ```
#[macro_export]
macro_rules! borders {
    ($($side:ident),* $(,)?) => {
        $crate::enums::Border::NONE $(| $crate::enums::Border::$side)*
    };
}

/// Makes creating help easier
///
/// ### Header:
/// - Printed in green
/// ```ignore
/// "header":
/// ```
///
/// ### Command help:
/// - Command is printed in yellow and has left padding of 2
/// - Others in default color
/// - Description has left padding of 4
/// ```ignore
/// // One description literals
/// "command" ["params"]* => "description"
/// // Multiple description literals
/// "command" ["params"]* => {
///     "description1",
///     "description2"
/// }
/// ```
///
/// ## Usage:
/// ```rust
/// use termint::help;
///
/// help!(
///     "Usage":
///     "-t" ["value"] => "Tests program with [value]"
///     "-d" => {
///         "Creates documentation",
///         "When used with -t, also tests the documentation",
///     }
///     "Special case":
///     "-ntd" => "Creates documentation and runs test without testing docs"
/// );
/// ```
#[macro_export]
macro_rules! help {
    // Rule for parsing header
    ($header:literal: $($rest:tt)*) => {
        println!(
            "{}:",
            $crate::style::Stylize::fg(
                $crate::widgets::Span::new($header),
                $crate::enums::Color::Green
            )
        );
        help!($($rest)*);
    };

    // Rule for parsing command help without curly braces
    (
        $cmd:literal $([$param:literal])* => $description:literal
        $($rest:tt)*
    ) => {
        print!(
            "  {}",
            $crate::style::Stylize::fg(
                $crate::widgets::Span::new($cmd),
                $crate::enums::Color::Yellow
            )
        );
        $(print!(" [{}]", $param);)*
        println!();
        println!("    {}", $description);
        help!($($rest)*);
    };

    // Rule for parsing command help with curly braces
    (
        $cmd:literal $([$param:literal])* => {
            $($description:literal),* $(,)?
        }
        $($rest:tt)*
    ) => {
        print!(
            "  {}",
            $crate::style::Stylize::fg(
                $crate::widgets::Span::new($cmd),
                $crate::enums::Color::Yellow
            )
        );
        $(print!(" [{}]", $param);)*
        println!();
        $(println!("    {}", $description);)*
        help!($($rest)*);
    };

    () => {};
}

/// Creates vector with given given Modifiers
///
/// ## Usage:
/// ```rust
/// # use termint::{enums::Modifier, modifiers};
/// // Without macro:
/// let mods = Modifier::BOLD | Modifier::ITALIC;
/// // With macro:
/// let mods = modifiers!(BOLD, ITALIC);
/// ```
#[macro_export]
macro_rules! modifiers {
    ($($mod:ident),* $(,)?) => {
        $crate::enums::Modifier::NONE $(| $crate::enums::Modifier::$mod)*
    };
}

/// Creates new paragraph in simpler way, when using heterogeneous types.
///
/// ## Usage:
/// ```rust
/// use termint::{prelude::*, paragraph, text::Text, widgets::Grad};
///
/// // Without macro using multiple types
/// let items: Vec<Box<dyn Text>> = vec![
///     Box::new("Paragraph".to_span()),
///     Box::new("macro".fg(Color::Red)),
///     Box::new(Grad::new("showcase", (0, 0, 255), (0, 255, 0)))
/// ];
/// let p = Paragraph::new(items);
///
/// // With macro:
/// let p = paragraph!(
///     "Paragraph",
///     "macro".fg(Color::Red),
///     Grad::new("showcase", (0, 0, 255), (0, 255, 0))
/// );
/// ```
#[macro_export]
macro_rules! paragraph {
    ($($text:expr),* $(,)?) => {
        $crate::widgets::Paragraph::new(vec![
            $({
                let t: Box<dyn $crate::text::Text> = $text.into();
                t
            }),*
        ])
    };
}

/// Macro to combine [`Border`] sides
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
/// # use termint::{enums::Color, help, widgets::ToSpan};
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
            $crate::widgets::Span::new($header).fg($crate::enums::Color::Green)
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
            $crate::widgets::Span::new($cmd).fg($crate::enums::Color::Yellow)
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
            $crate::widgets::Span::new($cmd).fg($crate::enums::Color::Yellow)
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

/// Creates new paragraph in more simple way
///
/// ## Usage:
/// ```rust
/// # use termint::{
/// #     enums::Color,
/// #     paragraph,
/// #     widgets::{Paragraph, ToSpan},
/// # };
/// // Without macro:
/// let p = Paragraph::new(vec![
///     Box::new("Macro".to_span()),
///     Box::new("test".fg(Color::Red))
/// ]);
/// // With macro:
/// let p = paragraph!(
///     "Macro".to_span(),
///     "test".fg(Color::Red)
/// );
/// ```
#[macro_export]
macro_rules! paragraph {
    ($($text:expr),* $(,)?) => {
        $crate::widgets::Paragraph::new(vec![
            $(Box::new($text)),*
        ])
    };
}

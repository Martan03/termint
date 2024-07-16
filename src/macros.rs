/// Macro to combine [`Border`] sides
///
/// ## Usage:
/// ```ignore
/// # use termint::{borders, widgets::border::Border};
/// // Without macro:
/// let top_left_right = Border::TOP | Border::LEFT | Border::RIGHT;
/// // With macro:
/// let top_left_right = borders!(TOP, LEFT, RIGHT);
/// ```
#[macro_export]
macro_rules! borders {
    ($($border:ident),*) => {
        $(Border::$border |)* 0
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
/// ```ignore
/// # use termint::{enums::fg::Fg, help, widgets::span::StrSpanExtension};
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
        println!("{}:", $header.fg(Fg::Green));
        help!($($rest)*);
    };

    // Rule for parsing command help without curly braces
    (
        $cmd:literal $([$param:literal])* => $description:literal
        $($rest:tt)*
    ) => {
        print!("  {}", $cmd.fg(Fg::Yellow));
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
        print!("  {}", $cmd.fg(Fg::Yellow));
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
/// ```ignore
/// # use termint::{enums::modifier::Modifier, mods};
/// // Without macro:
/// let mods = vec![Modifier::Bold, Modifier::Italic];
/// // With macro:
/// let mods = mods!(Bold, Italic);
/// ```
#[macro_export]
macro_rules! mods {
    ($($mod:ident),*) => {
        vec![$(Modifier::$mod, )*]
    };
}

/// Creates new paragraph in more simple way
///
/// ## Usage:
/// ```ignore
/// # use termint::{
/// #     enums::fg::Fg,
/// #     paragraph,
/// #     widgets::{paragraph::Paragraph, span::StrSpanExtension, grad::Grad},
/// # };
/// // Without macro:
/// let p = Paragraph::new(vec![
///     Box::new("Macro".to_span()),
///     Box::new("test".fg(Fg::Red))
/// ]);
/// // With macro:
/// let p = paragraph!(
///     Grad::new("Macro", (0, 120, 255), (120, 255, 0)),
///     "test".fg(Fg::Red)
/// );
/// ```
#[macro_export]
macro_rules! paragraph {
    ($($text:expr),* $(,)?) => {
        Paragraph::new(vec![
            $(Box::new($text)),*
        ])
    };
}

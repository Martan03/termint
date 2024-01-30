#[macro_export]
macro_rules! create_help {
    ($($cmd:literal => {$($description:literal),*}),* $(,)?) => {
        $(
            println!("  {}{}\x1b[0m", Fg::Yellow, $cmd);
            $(
                println!("    {}", $description);
            )*
        )*
    };
}

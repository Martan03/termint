extern crate termint;

#[cfg(test)]
mod tests {
    use termint::{enums::modifier::Modifier, mods};

    #[allow(unused)]
    fn block_render() {
        let mods = mods!(Bold, Italic);
    }
}

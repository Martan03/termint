/// [`Constrain`] enum contains some constrains for when adjusting widgets
#[derive(Debug)]
pub enum Constrain {
    Length(usize),
    Percent(usize),
}

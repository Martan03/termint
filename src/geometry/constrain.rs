/// [`Constrain`] enum contains some constrains for when adjusting layout
#[derive(Debug)]
pub enum Constrain {
    Length(usize),
    Percent(usize),
}

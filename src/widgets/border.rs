pub struct Border(u8);

#[allow(unused)]
impl Border {
    pub const TOP: u8 = 0b0001;
    pub const RIGHT: u8 = 0b0010;
    pub const BOTTOM: u8 = 0b0100;
    pub const LEFT: u8 = 0b1000;

    pub const NONE: u8 = 0b0000;
    pub const ALL: u8 = 0b1111;
}

pub const UP: u8 = 0b1000;
pub const LEFT: u8 = 0b0100;
pub const DOWN: u8 = 0b0010;
pub const RIGHT: u8 = 0b0001;

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum Pipe {
    None       = 0,
    Horizontal = LEFT | RIGHT,
    Vertical   = UP   | DOWN,
    UpLeft     = UP   | LEFT,
    UpRight    = UP   | RIGHT,
    DownLeft   = DOWN | LEFT,
    DownRight  = DOWN | RIGHT,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Down = 0,
    Up,
    Right,
    Left,
}

pub const DIRECTIONS: usize = 4;

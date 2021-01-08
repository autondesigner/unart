#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Down = 0,
    Up,
    Right,
    Left,
}

pub const RIGHT: usize = 0;
pub const DOWN: usize = 1;
pub const LEFT: usize = 2;
pub const UP: usize = 3;
pub const DIRECTIONS: usize = 4;

impl Direction {
    pub fn new(index: usize) -> Direction {
        match index {
            DOWN => Direction::Down,
            RIGHT => Direction::Right,
            UP => Direction::Up,
            LEFT => Direction::Left,
            _ => panic!("invalid direction index"),
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Direction::Down => DOWN,
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Left => LEFT,
        }
    }
    pub fn inverse(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
    pub fn complement(&self) -> Direction {
        match self {
            Direction::Down => Direction::Right,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

use crate::buildable::*;

pub struct Cell {
    pub level: usize,
}

impl Buildable for Cell {
    fn new() -> Self {
        let level = 0;
        Cell { level }
    }
}

use crate::buildable::*;

pub struct Tape<T: Buildable> {
    pub cells: Vec<T>,
}

impl<T: Buildable> Tape<T> {
    pub fn new(size: usize) -> Tape<T> {
        let mut cells = Vec::with_capacity(size);
        for _i in 0..size {
            cells.push(T::new());
        }
        Tape { cells }
    }
}

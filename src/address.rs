#[derive(Copy, Clone)]
pub struct Address {
    pub row: usize,
    pub column: usize,
}

impl Address {
    pub fn new(row: usize, column: usize) -> Address {
        Address { row, column }
    }
}

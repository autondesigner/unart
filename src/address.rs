#[derive(Copy, Clone)]
pub struct Address {
    pub row: usize,
    pub column: usize,
}

impl Address {
    pub fn new(row: usize, column: usize) -> Address {
        Address { row, column }
    }
    pub fn get_usize_radius(&self, address: Address) -> usize {
        let x;
        if self.column > address.column {
            x = self.column - address.column;
        } else {
            x = address.column - self.column;
        }
        let y;
        if self.row > address.row {
            y = self.row - address.row;
        } else {
            y = address.row - self.row;
        }
        x + y
    }
    pub fn get_radius(&self, address: Address) -> f64 {
        let x;
        if self.column > address.column {
            x = self.column - address.column;
        } else {
            x = address.column - self.column;
        }
        let y;
        if self.row > address.row {
            y = self.row - address.row;
        } else {
            y = address.row - self.row;
        }
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
}

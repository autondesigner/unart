use crate::address::*;
use crate::direction::*;

pub struct AddressBook {
    pub address: Address,
    pub neighbors: [Address; DIRECTIONS],
}

impl AddressBook {
    pub fn new(height: usize, width: usize, row: usize, column: usize) -> AddressBook {
        let address = Address::new(row, column);
        let down = Address::new((row + 1) % height, column);
        let up = Address::new((row + height - 1) % height, column);
        let right = Address::new(row, (column + 1) % width);
        let left = Address::new(row, (column + width - 1) % width);
        let neighbors = [down, up, right, left];
        AddressBook { address, neighbors }
    }
}

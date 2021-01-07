use crate::address::*;
use crate::address_book::*;
use crate::buildable::*;
use crate::direction::*;

pub struct Torus<T: Buildable> {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<T>,
    pub books: Vec<AddressBook>,
}

impl<T: Buildable> Torus<T> {
    pub fn new(width: usize, height: usize) -> Torus<T> {
        let cells = Torus::<T>::build_cells(width, height);
        let books = Torus::<T>::build_books(width, height);
        Torus {
            width,
            height,
            cells,
            books,
        }
    }
    pub fn build_cells(width: usize, height: usize) -> Vec<T> {
        let mut cells = Vec::with_capacity(width * height);
        for _i in 0..width * height {
            cells.push(T::new());
        }
        cells
    }
    pub fn build_books(width: usize, height: usize) -> Vec<AddressBook> {
        let mut books = Vec::with_capacity(width * height);
        for row in 0..height {
            for column in 0..width {
                books.push(AddressBook::new(height, width, row, column));
            }
        }
        books
    }
    pub fn index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }
    pub fn address_index(&self, address: Address) -> usize {
        address.row * self.width + address.column
    }
    pub fn find_neighbor_index(&self, address: Address, direction: Direction) -> usize {
        let neighbor = self.find_neighbor(address, direction);
        self.address_index(neighbor)
    }
    pub fn find_neighbor(&self, address: Address, direction: Direction) -> Address {
        let index = self.index(address.row, address.column);
        self.books[index].neighbors[direction as usize]
    }
    pub fn find_neighbor_no_torus(
        &self,
        address: Address,
        direction: Direction,
    ) -> Option<Address> {
        match direction {
            Direction::Down => {
                if address.row == self.height - 1 {
                    return None;
                }
                Some(Address::new(address.row + 1, address.column))
            }
            Direction::Up => {
                if address.row == 0 {
                    return None;
                }
                Some(Address::new(address.row - 1, address.column))
            }
            Direction::Right => {
                if address.column == self.width - 1 {
                    return None;
                }
                Some(Address::new(address.row, address.column + 1))
            }
            Direction::Left => {
                if address.column == 0 {
                    return None;
                }
                Some(Address::new(address.row, address.column - 1))
            }
        }
    }
    pub fn find_rule_neighborhood(&self, address: Address) -> Vec<Address> {
        let mut neighborhood = Vec::with_capacity(5);
        neighborhood.push(address);
        neighborhood.push(self.find_neighbor(address, Direction::Down));
        neighborhood.push(self.find_neighbor(address, Direction::Up));
        neighborhood.push(self.find_neighbor(address, Direction::Right));
        neighborhood.push(self.find_neighbor(address, Direction::Left));
        neighborhood
    }
    pub fn find_neighborhood(&self, address: Address, height: usize, width: usize) -> Vec<Address> {
        let mut wildcard = address;
        for _i in 0..width {
            wildcard = self.find_neighbor(wildcard, Direction::Left);
        }
        for _i in 0..height {
            wildcard = self.find_neighbor(wildcard, Direction::Up);
        }
        let neighborhood_height = height * 2 + 1;
        let neighborhood_width = width * 2 + 1;
        let neighborhood_cap = neighborhood_height * neighborhood_width;
        let mut neighborhood = Vec::with_capacity(neighborhood_cap);
        let mut direction = Direction::Right;
        for row in 0..neighborhood_height {
            for column in 0..neighborhood_width {
                neighborhood.push(wildcard);
                if column == width * 2 {
                    wildcard = self.find_neighbor(wildcard, Direction::Down);
                } else {
                    wildcard = self.find_neighbor(wildcard, direction);
                }
            }
            if direction == Direction::Right {
                direction = Direction::Left;
            } else {
                direction = Direction::Right;
            }
        }
        neighborhood
    }
}

use crate::address::*;
use crate::cell::*;
use crate::direction::*;
use crate::torus::*;

use std::collections::HashMap;

pub struct ImageTorus {
    pub height: usize,
    pub width: usize,
    pub torus: Torus<Cell>,
    pub back_torus: Torus<Cell>,
}

impl ImageTorus {
    pub fn new(width: usize, height: usize) -> ImageTorus {
        let torus = Torus::new(width, height);
        let back_torus = Torus::new(width, height);
        ImageTorus {
            width,
            height,
            torus,
            back_torus,
        }
    }
    pub fn set(&mut self, row: usize, column: usize, level: usize) {
        let index = self.torus.index(row, column);
        self.torus.cells[index].level = level;
    }
    pub fn set_address(&mut self, address: Address, level: usize) {
        let index = self.torus.address_index(address);
        self.torus.cells[index].level = level;
    }
    pub fn address_set(&mut self, address: Address, level: usize) {
        self.set(address.row, address.column, level);
    }
    pub fn address_iterate(&mut self, address: Address, levels: usize) -> usize {
        let index = self.torus.address_index(address);
        self.torus.cells[index].level += 1;
        self.torus.cells[index].level %= levels;
        self.torus.cells[index].level
    }
    pub fn get(&self, row: usize, column: usize) -> usize {
        let index = self.torus.index(row, column);
        self.torus.cells[index].level
    }
    pub fn find_neighbor_level(&self, address: Address, direction: Direction) -> usize {
        let index = self.torus.find_neighbor_index(address, direction);
        self.torus.cells[index].level
    }
    pub fn fill_back_torus(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.torus.index(row, column);
                let level = self.torus.cells[index].level;
                self.back_torus.cells[index].level = level;
            }
        }
    }
    pub fn get_fashion(&self, neighborhood: Vec<Address>) -> usize {
        let mut level_counter = HashMap::<usize, usize>::new();
        let neighborhood_size = neighborhood.len();
        for i in 0..neighborhood_size {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let key = self.back_torus.cells[index].level;
            *level_counter.entry(key).or_insert(0) += 1;
        }
        let mut level = 0;
        let mut maximum_repetitions = 0;
        for (key, repetitions) in level_counter {
            if repetitions > maximum_repetitions {
                maximum_repetitions = repetitions;
                level = key;
            }
        }
        level
    }
    pub fn get_less_fashion(&self, neighborhood: Vec<Address>) -> usize {
        let mut level_counter = HashMap::<usize, usize>::new();
        let neighborhood_size = neighborhood.len();
        for i in 0..neighborhood_size {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let key = self.back_torus.cells[index].level;
            *level_counter.entry(key).or_insert(0) += 1;
        }
        let mut level = 0;
        let mut minimum_repetitions = neighborhood_size + 1;
        for (key, repetitions) in level_counter {
            if repetitions < minimum_repetitions {
                minimum_repetitions = repetitions;
                level = key;
            }
        }
        level
    }
    pub fn get_less_fashion_overall(
        &self,
        neighborhood: Vec<Address>,
        level_counter: &Vec<(usize, usize)>,
    ) -> usize {
        for element in level_counter {
            let element_level = element.0;
            for address in &neighborhood {
                let index = self.torus.address_index(*address);
                let cell_level = self.back_torus.cells[index].level;
                if element_level == cell_level {
                    return element_level;
                }
            }
        }
        0
    }
    pub fn everything_zero(&self, neighborhood: &Vec<Address>) -> bool {
        for neighbor in neighborhood {
            let index = self.back_torus.address_index(*neighbor);
            let level = self.back_torus.cells[index].level;
            if level != 0 {
                return false;
            }
        }
        true
    }
    pub fn fashion(&mut self, radius: usize) {
        self.fill_back_torus();
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.torus.index(row, column);
                let address = Address::new(row, column);
                let neighborhood = self.torus.find_neighborhood(address, radius, radius);
                if !self.everything_zero(&neighborhood) {
                    let level = self.get_fashion(neighborhood);
                    self.torus.cells[index].level = level;
                }
            }
        }
    }
    pub fn less_fashion(&mut self, radius: usize) {
        self.fill_back_torus();
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.torus.index(row, column);
                let address = Address::new(row, column);
                let neighborhood = self.torus.find_neighborhood(address, radius, radius);
                if !self.everything_zero(&neighborhood) {
                    let level = self.get_less_fashion(neighborhood);
                    self.torus.cells[index].level = level;
                }
            }
        }
    }
    pub fn less_fashion_overall(&mut self, radius: usize, levels: usize) {
        self.fill_back_torus();
        let mut level_counter = Vec::with_capacity(levels);
        for i in 0..levels {
            level_counter.push((i, 0));
        }
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.back_torus.index(row, column);
                let key = self.back_torus.cells[index].level;
                level_counter[key].1 += 1;
            }
        }
        level_counter.sort_by_key(|k| k.1);
        /*
        for elemnt in &level_counter {
            println!("level {}, count {}", elemnt.0, elemnt.1);
        }
        */
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.torus.index(row, column);
                let address = Address::new(row, column);
                let neighborhood = self.torus.find_neighborhood(address, radius, radius);
                if !self.everything_zero(&neighborhood) {
                    let level = self.get_less_fashion_overall(neighborhood, &level_counter);
                    self.torus.cells[index].level = level;
                }
            }
        }
    }
}

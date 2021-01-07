use crate::address::*;
use crate::cell::*;
use crate::color::*;
use crate::direction::*;
use crate::torus::*;
use image::{Rgb, RgbImage};
use rand::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ImageTorus {
    pub height: usize,
    pub width: usize,
    pub torus: Torus<Cell>,
    pub back_torus: Torus<Cell>,
    pub image: RgbImage,
    pub colors: Vec<Color>,
    pub levels: usize,
}

impl ImageTorus {
    pub fn new(
        width: usize,
        height: usize,
        rng: &mut StdRng,
        hues_count: usize,
        saturations_count: usize,
        values_count: usize,
    ) -> ImageTorus {
        let torus = Torus::new(width, height);
        let back_torus = Torus::new(width, height);
        let image = RgbImage::new(width as u32, height as u32);
        let levels = hues_count * saturations_count * values_count;
        let colors = build_colors(rng, hues_count, saturations_count, values_count);
        ImageTorus {
            levels,
            colors,
            image,
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
    pub fn radius_set(&mut self, address: Address, radius: usize, level: usize) {
        let neighborhood = self.torus.find_neighborhood(address, radius, radius);
        for neighbor in neighborhood {
            self.address_set(neighbor, level);
        }
    }
    pub fn address_iterate(&mut self, address: Address) -> usize {
        let index = self.torus.address_index(address);
        self.torus.cells[index].level += 1;
        self.torus.cells[index].level %= self.levels;
        self.torus.cells[index].level
    }
    pub fn address_iterate_level(&mut self, address: Address, level: usize) -> usize {
        let index = self.torus.address_index(address);
        self.torus.cells[index].level += level;
        self.torus.cells[index].level %= self.levels;
        self.torus.cells[index].level
    }
    pub fn address_get(&self, address: Address) -> usize {
        let index = self.torus.address_index(address);
        self.torus.cells[index].level
    }
    pub fn get(&self, row: usize, column: usize) -> usize {
        let index = self.torus.index(row, column);
        self.torus.cells[index].level
    }
    pub fn get_aut_adder(&self, address: Address) -> usize {
        let mut level = self.address_get(address);
        let neighborhood = self.torus.find_rule_neighborhood(address);
        for neighbor in neighborhood {
            level += self.address_get(neighbor);
            level %= self.levels;
        }
        level
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
    pub fn get_fashion_hash(&self, neighborhood: Vec<Address>) -> usize {
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
            } else if repetitions == maximum_repetitions {
                if key < level {
                    level = key;
                }
            }
        }
        level
    }
    pub fn get_fashion(&self, neighborhood: &Vec<Address>) -> usize {
        let mut level_counter = Vec::with_capacity(self.levels);
        for _i in 0..self.levels {
            level_counter.push(0);
        }
        let neighborhood_size = neighborhood.len();
        for i in 0..neighborhood_size {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let key = self.back_torus.cells[index].level;
            level_counter[key] += 1;
        }
        let mut level = 0;
        let mut maximum_repetitions = 0;
        for i in 0..self.levels {
            let repetitions = level_counter[i];
            if repetitions > maximum_repetitions {
                maximum_repetitions = repetitions;
                level = i;
            }
        }
        level
    }
    pub fn get_less_fashion(&self, neighborhood: &Vec<Address>) -> usize {
        let mut level_counter = Vec::with_capacity(self.levels);
        for _i in 0..self.levels {
            level_counter.push(0);
        }
        let neighborhood_size = neighborhood.len();
        for i in 0..neighborhood_size {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let key = self.back_torus.cells[index].level;
            level_counter[key] += 1;
        }
        let mut level = 0;
        let mut minimum_repetitions = usize::MAX;
        for i in 0..self.levels {
            let repetitions = level_counter[i];
            if repetitions < minimum_repetitions && repetitions > 0 {
                minimum_repetitions = repetitions;
                level = i;
            }
        }
        level
    }
    pub fn get_less_fashion_hash(&self, neighborhood: Vec<Address>) -> usize {
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
            } else if repetitions == minimum_repetitions {
                if key < level {
                    level = key;
                }
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
            let index = self.torus.address_index(*neighbor);
            let level = self.torus.cells[index].level;
            if level != 0 {
                return false;
            }
        }
        true
    }
    pub fn everything_back_zero(&self, neighborhood: &Vec<Address>) -> bool {
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
                let index = self.back_torus.index(row, column);
                let address = Address::new(row, column);
                let neighborhood = self.back_torus.find_neighborhood(address, radius, radius);
                if !self.everything_back_zero(&neighborhood) {
                    let level = self.get_fashion(&neighborhood);
                    self.torus.cells[index].level = level;
                }
            }
        }
    }
    pub fn fashion_neighborhood(&mut self, radius: usize, neighborhood: &Vec<Address>) {
        self.fill_back_torus();
        for i in 0..neighborhood.len() {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let neighborhood_rec = self.back_torus.find_neighborhood(address, radius, radius);
            let level = self.get_fashion(&neighborhood_rec);
            self.torus.cells[index].level = level;
        }
    }
    pub fn adder_neighborhood(&mut self, neighborhood: &Vec<Address>) {
        self.fill_back_torus();
        for i in 0..neighborhood.len() {
            let address = neighborhood[i];
            let neighborhood_rec = self.back_torus.find_rule_neighborhood(address);
            let mut level = 0;
            for neigbor in neighborhood_rec {
                level += self.back_torus.cells[self.back_torus.address_index(neigbor)].level;
                level %= self.levels;
            }
            //println!("level {}", level);
            self.address_set(address, level);
        }
    }
    pub fn less_fashion_neighborhood(&mut self, radius: usize, neighborhood: &Vec<Address>) {
        self.fill_back_torus();
        for i in 0..neighborhood.len() {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let neighborhood_rec = self.back_torus.find_neighborhood(address, radius, radius);
            let level = self.get_less_fashion(&neighborhood_rec);
            self.torus.cells[index].level = level;
        }
    }
    pub fn less_fashion_overall_neighborhood(
        &mut self,
        radius: usize,
        neighborhood: &Vec<Address>,
    ) {
        self.fill_back_torus();
        let mut level_counter = Vec::with_capacity(self.levels);
        for i in 0..self.levels {
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
        for i in 0..neighborhood.len() {
            let address = neighborhood[i];
            let index = self.back_torus.address_index(address);
            let neighborhood_rec = self.torus.find_neighborhood(address, radius, radius);
            let level = self.get_less_fashion_overall(neighborhood_rec, &level_counter);
            self.torus.cells[index].level = level;
        }
    }
    pub fn less_fashion(&mut self, radius: usize) {
        self.fill_back_torus();
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.back_torus.index(row, column);
                let address = Address::new(row, column);
                let neighborhood = self.back_torus.find_neighborhood(address, radius, radius);
                let level = self.get_less_fashion(&neighborhood);
                self.torus.cells[index].level = level;
            }
        }
    }
    pub fn save_image(&mut self, folder_path: &Path, image_index: usize) {
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.get(row, column);
                //let color = self.colors[color_index];
                let red = self.colors[level].rgb[0];
                let green = self.colors[level].rgb[1];
                let blue = self.colors[level].rgb[2];
                //println!("color: {} {} {}", red, green, blue);
                self.image
                    .put_pixel(column as u32, row as u32, Rgb([red, green, blue]));
            }
        }
        let absolute_path = folder_path.join(format!("picture_{}.png", image_index));
        self.image
            .save(absolute_path)
            .expect("Could not save the image");
    }
    pub fn less_fashion_overall(&mut self, radius: usize) {
        self.fill_back_torus();
        let mut level_counter = Vec::with_capacity(self.levels);
        for i in 0..self.levels {
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

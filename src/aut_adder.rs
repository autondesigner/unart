use crate::address::*;
use crate::color::*;
use crate::image_torus::*;
use crate::util::*;
use image::{Rgb, RgbImage};
use rand::prelude::*;
use std::fs;
use std::path::Path;

pub struct AutAdder {
    pub tori_size: usize,
    pub tori: Vec<ImageTorus>,
    pub back_tori: Vec<ImageTorus>,
    pub height: usize,
    pub width: usize,
    pub levels: usize,
    pub colors: Vec<Color>,
    pub rng: StdRng,
}

impl AutAdder {
    pub fn new() -> AutAdder {
        let mut rng = time_seeded_rng();
        let height = 256 + 128;
        let width = height * 2;
        let tori_size = 2;
        let tori = AutAdder::build_tori(tori_size, width, height);
        let back_tori = AutAdder::build_tori(tori_size, width, height);
        let levels = 12;
        let colors = build_colors(&mut rng, levels);
        AutAdder {
            rng,
            height,
            width,
            tori_size,
            tori,
            back_tori,
            levels,
            colors,
        }
    }
    pub fn build_tori(tori_size: usize, width: usize, height: usize) -> Vec<ImageTorus> {
        let mut tori = Vec::with_capacity(tori_size);
        for _i in 0..tori_size {
            tori.push(ImageTorus::new(width, height));
        }
        tori
    }
    pub fn fill_first_torus(&mut self) {
        /*
                self.tori[0].set(0, 0, 1);
                self.tori[0].set(0, self.width - 1, 1);
                self.tori[0].set(self.height - 1, 0, 1);
                self.tori[0].set(self.height - 1, self.width - 1, 1);
        */
        self.tori[0].set(self.height / 2, self.width / 2, 1);
        self.tori[0].set(self.height / 2, self.width / 2 - 1, 1);
        self.tori[0].set(self.height / 2 - 1, self.width / 2, 1);
        self.tori[0].set(self.height / 2 - 1, self.width / 2 - 1, 1);

        /*
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.rng.gen_range(0..self.levels);
                self.tori[0].set(row, column, level);
            }
        }
        */
    }
    pub fn fill_back_torus(&mut self, index: usize) {
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.tori[index].get(row, column);
                self.back_tori[index].set(row, column, level);
            }
        }
    }
    pub fn iterate(&mut self) {
        self.fill_back_torus(0);
        for row in 0..self.height {
            for column in 0..self.width {
                let address = Address::new(row, column);
                let mut level = 0;
                let neighborhood = self.back_tori[0].torus.find_rule_neighborhood(address);
                for neighbor in &neighborhood {
                    level += self.back_tori[0].address_get(*neighbor);
                    level %= self.levels;
                }
                self.tori[0].set(row, column, level);
                if !self.back_tori[0].everything_zero(&neighborhood) && level == 0 {
                    self.tori[1].address_iterate(address, self.levels);
                    //println!("ITERATE");
                }
            }
        }
    }
    pub fn render(&mut self) {
        let render_path = Path::new("render");
        let folder_path = render_path.join("a0");
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        fs::remove_dir_all(&folder_path);
        fs::create_dir_all(&folder_path);
        self.fill_first_torus();
        //self.save_image(&mut image, &folder_path, 0);
        for i in 0..256 {
            println!("------------------pre iteration {}", i);
            self.iterate();
        }
        for i in 0..256 {
            println!("------------------iteration {}", i);
            self.iterate();
            self.tori[1].fashion(1, self.levels);
            //self.tori[1].less_fashion_overall(1, self.levels);
            self.tori[1].fashion(1, self.levels);
            self.save_image(&mut image, &folder_path, i);
        }
    }
    pub fn save_image(&mut self, image: &mut RgbImage, folder_path: &Path, image_index: usize) {
        //self.fill_image_torus();
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.tori[1].get(row, column);
                //let color = self.colors[color_index];
                let red = self.colors[level].rgb[0];
                let green = self.colors[level].rgb[1];
                let blue = self.colors[level].rgb[2];
                //println!("color: {} {} {}", red, green, blue);
                image.put_pixel(column as u32, row as u32, Rgb([red, green, blue]));
            }
        }
        let absolute_path = folder_path.join(format!("picture_{}.png", image_index));
        image.save(absolute_path).expect("Could not save the image");
    }
}

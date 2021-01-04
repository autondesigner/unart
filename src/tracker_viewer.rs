use crate::address::*;
use crate::color::*;
use crate::direction::*;
use crate::image_torus::*;
use crate::track::*;
use crate::util::*;
use image::{Rgb, RgbImage};
use rand::prelude::*;
use std::fs;
use std::path::Path;

pub struct TrackerViewer {
    pub rng: StdRng,
    pub width: usize,
    pub height: usize,
    pub colors: Vec<Color>,
    pub levels: usize,
    pub max_h_moves: usize,
    pub max_v_moves: usize,
    pub min_h_moves: usize,
    pub min_v_moves: usize,
    pub image_torus: ImageTorus,
    pub back_image_torus: ImageTorus,
    pub walkers: Vec<Address>,
    pub walkers_count: usize,
    pub tracks: Vec<Track>,
}

impl TrackerViewer {
    pub fn new(height: usize, levels: usize) -> TrackerViewer {
        let width = height * 2;
        let mut rng = time_seeded_rng();
        let min_moves = 12;
        let min_h_moves = min_moves;
        let min_v_moves = min_moves;
        let max_h_moves = min_h_moves * 2;
        let max_v_moves = min_v_moves * 2;
        let colors = build_colors(&mut rng, levels);
        let image_torus = ImageTorus::new(width, height);
        let back_image_torus = ImageTorus::new(width, height);
        let walkers_count = 1;
        let mut walkers = Vec::with_capacity(walkers_count);
        for _i in 0..walkers_count / 2 {
            walkers.push(Address::new(0, 0));
        }
        for _i in walkers_count / 2..walkers_count {
            walkers.push(Address::new(height / 2, width / 2));
        }
        let mut tracks = Vec::with_capacity(walkers_count);
        for _i in 0..walkers_count {
            tracks.push(Track::new(&mut rng, 1, 1));
        }
        let mut tracker_viewer = TrackerViewer {
            tracks,
            walkers,
            walkers_count,
            min_h_moves,
            min_v_moves,
            max_v_moves,
            max_h_moves,
            rng,
            levels,
            width,
            height,
            colors,
            image_torus,
            back_image_torus,
        };
        tracker_viewer.rebuild_tracks();
        tracker_viewer
    }
    pub fn rebuild_tracks(&mut self) {
        for i in 0..self.walkers_count {
            let h_moves = self.rng.gen_range(self.min_h_moves..self.max_h_moves);
            let v_moves = self.rng.gen_range(self.min_v_moves..self.max_v_moves);
            self.tracks[i] = Track::new(&mut self.rng, h_moves, v_moves);
        }
    }
    pub fn rebuild_track(&mut self, wi: usize) {
        let h_moves = self.rng.gen_range(self.min_h_moves..self.max_h_moves);
        let v_moves = self.rng.gen_range(self.min_v_moves..self.max_v_moves);
        self.tracks[wi] = Track::new(&mut self.rng, h_moves, v_moves);
    }
    pub fn render(&mut self, folder_path: &Path) {
        fs::remove_dir_all(folder_path);
        fs::create_dir_all(folder_path);
        self.render_picture(folder_path);
    }
    pub fn render_animation(&mut self, folder_path: &Path) {
        fs::remove_dir_all(folder_path);
        fs::create_dir_all(folder_path);
        self.animation(folder_path);
    }
    pub fn save_image(
        &mut self,
        image: &mut RgbImage,
        folder_path: &Path,
        image_index: &mut usize,
    ) {
        println!("picture {}", image_index);
        //self.fill_image_torus();
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.back_image_torus.get(row, column);
                //let color = self.colors[color_index];
                let red = self.colors[level].rgb[0];
                let green = self.colors[level].rgb[1];
                let blue = self.colors[level].rgb[2];
                image.put_pixel(column as u32, row as u32, Rgb([red, green, blue]));
            }
        }
        let absolute_path = folder_path.join(format!("picture_{}.png", image_index));
        image.save(absolute_path).expect("Could not save the image");
        *image_index += 1;
    }
    pub fn get_usize_radius(&self, wi: usize, address: Address) -> usize {
        let x;
        if self.walkers[wi].column > address.column {
            x = self.walkers[wi].column - address.column;
        } else {
            x = address.column - self.walkers[wi].column;
        }
        let y;
        if self.walkers[wi].row > address.row {
            y = self.walkers[wi].row - address.row;
        } else {
            y = address.row - self.walkers[wi].row;
        }
        x + y
    }
    pub fn get_radius(&self, wi: usize, address: Address) -> f64 {
        let x;
        if self.walkers[wi].column > address.column {
            x = self.walkers[wi].column - address.column;
        } else {
            x = address.column - self.walkers[wi].column;
        }
        let y;
        if self.walkers[wi].row > address.row {
            y = self.walkers[wi].row - address.row;
        } else {
            y = address.row - self.walkers[wi].row;
        }
        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
    pub fn fill_image_torus(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.back_image_torus.torus.index(row, column);
                let level = self.back_image_torus.torus.cells[index].level;
                self.image_torus.torus.cells[index].level = level;
            }
        }
    }
    pub fn zero(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.back_image_torus.torus.index(row, column);
                self.back_image_torus.torus.cells[index].level = 0;
            }
        }
    }
    pub fn sum_up(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let address = Address::new(row, column);
                let e = self.image_torus.get(row, column);
                let a = self
                    .image_torus
                    .find_neighbor_level(address, Direction::Down);
                let b = self.image_torus.find_neighbor_level(address, Direction::Up);
                let c = self
                    .image_torus
                    .find_neighbor_level(address, Direction::Right);
                let d = self
                    .image_torus
                    .find_neighbor_level(address, Direction::Left);

                self.image_torus
                    .set(row, column, (a + b + c + d + e) % self.levels);
            }
        }
    }
    pub fn animation(&mut self, folder_path: &Path) {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        let mut image_index = 0;
        let mut index_modulator = 0;
        let index_modulus = 256;
        let mut less_fashion_modulator = 0;
        let less_fashion_modulus = 16;
        let mut fashion_modulator = 0;
        let fashion_modulus = 1;
        let base_color = self.rng.gen_range(0..360);
        //let base_color = 180;
        let base_levels_count = 6;
        let mut base_levels = Vec::with_capacity(base_levels_count);
        for i in 0..base_levels_count {
            base_levels.push((base_color + i * self.levels / base_levels_count) % self.levels);
        }
        println!("strokes");
        let variations = 36;
        loop {
            self.zero();
            //println!("l {}", l);
            for i in 0..self.walkers_count {
                self.walkers[i] = Address::new(
                    self.rng.gen_range(0..self.height),
                    self.rng.gen_range(0..self.width),
                );
            }
            for j in 0..base_levels_count {
                //println!("j {}", j);
                let mut levels = Vec::with_capacity(variations);
                let base_level = base_levels[j];
                for k in 0..variations {
                    levels.push((base_level + k) % self.levels);
                }
                for k in 0..8 {
                    //println!("k {}", k);
                    let stroke_size = self.rng.gen_range(0..4);
                    let level = levels[self.rng.gen_range(0..levels.len())];
                    //println!("length {}", self.tracks[0].length);
                    for i in 0..self.tracks[0].length {
                        for wi in 0..self.walkers_count {
                            let neighborhood = self.back_image_torus.torus.find_neighborhood(
                                self.walkers[wi],
                                stroke_size,
                                stroke_size,
                            );

                            for neighbor in neighborhood {
                                if self.get_radius(wi, neighbor) <= stroke_size as f64 + 0.5 {
                                    self.back_image_torus.address_set(
                                        neighbor,
                                        (level + self.rng.gen_range(0..8)) % self.levels,
                                    );
                                }
                            }
                            self.walkers[wi] = self
                                .back_image_torus
                                .torus
                                .find_neighbor(self.walkers[wi], self.tracks[wi].moves[i]);
                            index_modulator += 1;
                            index_modulator %= index_modulus;
                            if index_modulator % index_modulus == 0 {
                                self.save_image(&mut image, folder_path, &mut image_index);
                                if image_index == 16 * 32 {
                                    return;
                                }
                            }
                        }
                    }
                    self.rebuild_tracks();
                }
            }
        }
    }

    pub fn render_picture(&mut self, folder_path: &Path) {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        let mut post_index = 0;
        let mut index_modulator = 0;
        let index_modulus = 256;
        loop {
            for wi in 0..self.walkers_count {
                let stroke_size = self.rng.gen_range(4..8);
                let level = self.rng.gen_range(1..self.levels);
                for i in 0..self.tracks[wi].length {
                    let neighborhood = self.back_image_torus.torus.find_neighborhood(
                        self.walkers[wi],
                        stroke_size,
                        stroke_size,
                    );

                    for neighbor in neighborhood {
                        if self.get_radius(wi, neighbor) <= stroke_size as f64 + 0.5 {
                            self.back_image_torus.address_iterate(neighbor, self.levels);
                            let index = self.back_image_torus.set_address(neighbor, level);
                        }
                    }
                    self.walkers[wi] = self
                        .back_image_torus
                        .torus
                        .find_neighbor(self.walkers[wi], self.tracks[wi].moves[i]);
                    index_modulator += 1;
                    index_modulator %= index_modulus;
                    if index_modulator % index_modulus == 0 {
                        for i in 0..self.walkers_count {
                            self.walkers[i] = Address::new(
                                self.rng.gen_range(0..self.height),
                                self.rng.gen_range(0..self.width),
                            );
                        }
                        println!("post_index {}", post_index);
                        //self.sum_up();
                        //self.image_torus.less_fashion(2);
                        //self.image_torus.fashion(1);
                        if post_index == 256 {
                            //self.image_torus.fashion(1);
                            for i in 0..4 {
                                //self.back_image_torus.fashion(1);
                                println!("less_fashion");
                                self.back_image_torus.less_fashion(1);
                                println!("fashion");
                                self.back_image_torus.fashion(1);
                                self.back_image_torus.fashion(1);
                            }
                            self.save_image(&mut image, folder_path, &mut 0);
                            return;
                        }
                        post_index += 1;
                    }
                }
                self.rebuild_track(wi);
            }
        }
    }
}

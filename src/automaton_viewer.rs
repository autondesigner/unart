use crate::automaton::*;
use crate::color::*;
use crate::image_torus::*;
use crate::util::*;
use image::{Rgb, RgbImage};
use std::fs;
use std::path::Path;

pub struct AutomatonViewer {
    pub width: usize,
    pub height: usize,
    pub automaton: Automaton,
    pub colors: Vec<Color>,
    pub image_torus: ImageTorus,
}

impl AutomatonViewer {
    pub fn new(height: usize, states_count: usize) -> AutomatonViewer {
        let width = height * 2;
        let mut rng = time_seeded_rng();
        let automaton = Automaton::new(&mut rng, width, states_count);
        let colors = build_colors(&mut rng, states_count);
        let image_torus = ImageTorus::new(width, height);
        AutomatonViewer {
            width,
            height,
            automaton,
            colors,
            image_torus,
        }
    }
    pub fn render(&mut self, folder_path: &Path, iterations: usize) {
        fs::remove_dir_all(folder_path);
        fs::create_dir_all(folder_path);
        for index in 0..iterations {
            println!("picture {}", index);
            self.iteration(folder_path, index);
        }
    }
    pub fn iteration(&mut self, folder_path: &Path, image_index: usize) {
        let mut image = RgbImage::new(self.width as u32, self.height as u32);
        for row in 0..self.height {
            self.automaton.iterate();
            for column in 0..self.width {
                let level = self.automaton.tape.cells[column].level;
                self.image_torus.set(row, column, level);
            }
        }
        for i in 0..1 {
            println!("fashion {}", i);
            self.image_torus.fashion(4);
        }
        for row in 0..self.height {
            for column in 0..self.width {
                let level = self.image_torus.get(row, column);
                //let color = self.colors[color_index];
                let red = self.colors[level].rgb[0];
                let green = self.colors[level].rgb[1];
                let blue = self.colors[level].rgb[2];
                image.put_pixel(column as u32, row as u32, Rgb([red, green, blue]));
            }
        }
        let absolute_path = folder_path.join(format!("picture_{}.png", image_index));
        image.save(absolute_path).expect("Could not save the image");
    }
}

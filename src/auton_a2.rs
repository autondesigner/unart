use crate::address::*;
use crate::direction::*;
use crate::fib_generator::*;
use crate::image_torus::*;
use crate::util::*;
use crate::walker::*;
use std::fs;
use std::path::Path;

pub struct AutonA2 {
    pub height: usize,
    pub width: usize,
    pub generator: FibGenerator,
    pub levels: usize,
    pub image: ImageTorus,
    pub addresses: Vec<Address>,
    pub walkers: Vec<Walker>,
}

impl AutonA2 {
    pub fn new() -> AutonA2 {
        let height = 256;
        let width = height * 2;
        let generator = FibGenerator::new();
        let mut rng = time_seeded_rng();
        let hues_count = 1;
        let values_count = 360;
        let saturations_count = 1;
        let levels = hues_count * saturations_count * values_count;
        let image = ImageTorus::new(
            width,
            height,
            &mut rng,
            hues_count,
            saturations_count,
            values_count,
        );
        let walker = Walker::new(Direction::Down, 1, 1);
        let mut addresses = Vec::new();
        addresses.push(Address::new(height / 2, width / 2));
        let mut walkers = Vec::new();
        walkers.push(Walker::new(Direction::Right, 1, 1));
        AutonA2 {
            width,
            walkers,
            height,
            generator,
            levels,
            image,
            addresses,
        }
    }
    pub fn render() {
        let mut auton = AutonA2::new();
        let render_path = Path::new("render");
        let folder_path = render_path.join("a0");
        fs::remove_dir_all(&folder_path);
        fs::create_dir_all(&folder_path);
        let iterations = 1;
        let pictures = 1024;
        let min_length = 1;
        let max_length = 4;
        let max_v_moves = 1;
        let max_h_moves = max_v_moves;
        let max_walkers = 128;
        let mut die_rate = Vec::new();
        let mut last_addresses = Vec::new();
        let max_last_addresses_len = 512;
        for _i in 0..3 {
            die_rate.push(false);
        }
        for _i in 0..1 {
            die_rate.push(true);
        }
        for picture in 0..pictures {
            println!("---------------picture {}", picture);
            for iteration in 0..iterations {
                //println!("iteration {}", iteration);
                for wi in 0..auton.walkers.len() {
                    let square_length = auton.walkers[wi].length();
                    //println!("square_length {}", square_length);
                    let level = auton.walkers[wi].level();
                    let neighborhood = auton.image.torus.find_rectangle_neighborhood(
                        auton.addresses[wi],
                        square_length,
                        square_length,
                    );
                    for address in neighborhood {
                        auton.image.address_iterate(address);
                    }
                    auton.addresses[wi] = auton
                        .image
                        .torus
                        .find_neighbor(auton.addresses[wi], auton.walkers[wi].direction());
                    auton.walkers[wi].iterate();
                    last_addresses.push(auton.addresses[wi]);
                }
                while last_addresses.len() > max_last_addresses_len {
                    last_addresses.remove(0);
                }
                if auton.walkers.len() < max_walkers {
                    let direction = Direction::new(auton.generate() % DIRECTIONS);
                    let level = auton.generate() % auton.levels;
                    let length = auton.generate() % (max_length + 1 - min_length) + min_length;
                    auton.walkers.push(Walker::new(direction, level, length));
                    let index = auton.generate() % last_addresses.len();
                    auton.addresses.push(last_addresses[index]);
                }
                if auton.walkers.len() > 1 {
                    if die_rate[auton.generate() % die_rate.len()] {
                        let index = auton.generate() % auton.walkers.len();
                        auton.walkers.remove(index);
                        auton.addresses.remove(index);
                    }
                }
                for wi in 0..auton.walkers.len() {
                    let direction = Direction::new(auton.generate() % DIRECTIONS);
                    let level = auton.generate() % auton.levels;
                    let length = auton.generate() % (max_length + 1 - min_length) + min_length;
                    auton.walkers[wi].add_element(direction, level, length);
                }
                auton.generator.iterate();
            }
            auton.image.save_image(&folder_path, picture);
        }
    }
    pub fn generate(&mut self) -> usize {
        self.generator.generate()
    }
}

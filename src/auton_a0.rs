use crate::address::*;
use crate::direction::*;
use crate::image_torus::*;
use crate::track::*;
use crate::util::*;
use rand::prelude::*;
use std::fs;
use std::path::Path;
pub struct AutonA0 {
	pub height: usize,
	pub width: usize,
	pub image_torus: ImageTorus,
	pub levels: usize,
	pub rng: StdRng,
	pub tracks: Vec<Track>,
}

impl AutonA0 {
	pub fn new() -> AutonA0 {
		let height = 256;
		let width = height * 2;
		let levels = 24;
		let mut rng = time_seeded_rng();
		let image_torus = ImageTorus::new(width, height, levels, &mut rng);
		let mut tracks = Vec::new();
		AutonA0 {
			tracks,
			rng,
			levels,
			height,
			width,
			image_torus,
		}
	}
	pub fn fractal(
		&mut self,
		folder_path: &Path,
		address: Address,
		iteration: &mut usize,
		fib: [usize; 2],
	) {
		let level = self.image_torus.address_get(address);
		let radius = ((level + 1) * 2 + 0) % 12;
		let neighborhood = self
			.image_torus
			.torus
			.find_neighborhood(address, radius, radius);
		for neighbor in &neighborhood {
			self.image_torus.address_iterate(*neighbor);
		}
		let modulus = 1;
		if *iteration % modulus == 0 {
			println!("iteration {}", *iteration / modulus);
			self.image_torus
				.save_image(folder_path, *iteration / modulus);
		}
		if *iteration >= modulus * 1024 {
			return;
		}
		*iteration += 1;
		let direction_index = (level + 0) % 4;
		//direction_index = fib[1] % 4;
		let direction = Direction::new(direction_index);
		let rec_fib = fib[0] + fib[1];
		let mut rec_address = address;
		for i in 0..level * fib[1] % 64 {
			rec_address = self.image_torus.torus.find_neighbor(rec_address, direction);
		}
		self.fractal(folder_path, rec_address, iteration, [fib[1], rec_fib]);
	}
	pub fn render(&mut self) {
		let render_path = Path::new("render");
		let folder_path = render_path.join("a0");
		fs::remove_dir_all(&folder_path);
		fs::create_dir_all(&folder_path);
		let mut radius = 0;
		let address = Address::new(self.height / 2, self.width / 2);

		let mut iteration = 0;
		self.fractal(&folder_path, address, &mut iteration, [0, 1]);
		//self.image_torus.less_fashion_overall(1);
		//self.image_torus.less_fashion(1);
		//self.image_torus.fashion(4);
		//self.image_torus.save_image(&folder_path, 0)
	}
}

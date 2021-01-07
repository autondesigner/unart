use crate::address::*;
use crate::direction::*;
use crate::image_torus::*;
use crate::util::*;
use crate::walker::*;
use std::fs;
use std::path::Path;

pub struct AutonA1 {
	walkers: Vec<Walker>,
	image: ImageTorus,
}

pub fn iterate_fibn(fib: &mut Vec<usize>) {
	let length = fib.len();
	let mut next_fib = fib[0];
	for i in 1..length {
		next_fib = next_fib.wrapping_add(fib[i]);
	}
	for i in 0..length - 1 {
		//next_fib = next_fib.wrapping_add(fib[i]);
		fib[i] = fib[i + 1];
	}
	fib[length - 1] = next_fib;
}

impl AutonA1 {
	pub fn render() {
		let render_path = Path::new("render");
		let folder_path = render_path.join("a0");
		fs::remove_dir_all(&folder_path);
		fs::create_dir_all(&folder_path);
		let hues_count = 12;
		let values_count = 12;
		let saturations_count = 12;
		let levels = hues_count * saturations_count * values_count;
		let height = 128 + 64;
		let width = height * 2;
		let max_radius = 16;
		let mut rng = time_seeded_rng();
		let mut image = ImageTorus::new(
			width,
			height,
			&mut rng,
			hues_count,
			saturations_count,
			values_count,
		);
		let mut last_address = Address::new(height / 2, width / 2);
		let mut addresses = Vec::new();
		let mut checkpoints = Vec::new();
		//addresses.push(last_address);
		checkpoints.push(last_address);
		let mut walkers = Vec::new();
		let mut fib2 = vec![0, 1];
		let mut fib3 = vec![0, 0, 1];
		let mut fib4 = vec![0, 0, 0, 1];
		let mut fib5 = vec![0, 0, 0, 0, 1];
		let mut fib6 = vec![0, 0, 0, 0, 0, 1];
		let mut born = Vec::new();
		for i in 0..6 {
			born.push(false);
		}
		for i in 0..5 {
			born.push(true);
		}
		let mut use_last_address = Vec::new();
		for i in 0..1 {
			use_last_address.push(false);
		}
		for i in 0..1 {
			use_last_address.push(true);
		}
		for elem in 0..0 {
			panic!("wrong");
		}
		let mut directions = Vec::new();

		for i in 0..1 {
			directions.push(DOWN);
		}
		for i in 0..1 {
			directions.push(RIGHT);
		}
		for i in 0..1 {
			directions.push(UP);
		}
		for i in 0..1 {
			directions.push(LEFT);
		}
		let mut increment = Vec::new();
		for i in 0..4 {
			increment.push(true);
		}
		for i in 0..1 {
			increment.push(false);
		}
		let mut checkpoints_remover = Vec::new();
		for i in 0..1 {
			checkpoints_remover.push(true);
		}
		for i in 0..0 {
			checkpoints_remover.push(false);
		}
		let mut set_over_iterate = Vec::new();
		for i in 0..1 {
			set_over_iterate.push(true);
		}
		for i in 0..1 {
			set_over_iterate.push(false);
		}
		let directions_power_variations = 4;
		let directions_min_power = 1;
		let directions_power_factor = 16;
		let mut directions_counter = [0, 0, 0, 0];
		let directions_counter_modulus = 16;
		//walkers.push(Walker::new(Direction::Down, 1, 0));
		let iterations = 4096;
		let iteration_modulus = 1;
		let directions_size_limit = 1024;
		for iteration in 0..iterations * iteration_modulus {
			let mut walkers_lenght = walkers.len();
			while walkers_lenght > 4 {
				walkers.remove(0);
				addresses.remove(0);
				walkers_lenght = walkers.len();
			}
			if born[fib5[1] % born.len()] {
				if use_last_address[fib5[3] % use_last_address.len()] {
					addresses.push(last_address);
				} else {
					addresses.push(checkpoints[fib5[2] % checkpoints.len()]);
				}
				let direction = Direction::new(directions[fib3[0] % directions.len()]);
				if directions.len() < directions_size_limit {
					let direction_index = direction.index();
					directions_counter[direction_index] += 1;
					if directions_counter[direction_index] == directions_counter_modulus {
						directions_counter[direction_index] = 0;
						directions.push(direction.inverse().index());
					}
				}
				walkers.push(Walker::new(
					direction,
					fib3[1] % levels,
					fib3[2] % max_radius,
				));
			}
			/*
			for wi in 0..walkers.len() {
				let direction = Direction::new(directions[fib4[1] % directions.len()]);
				if directions.len() < directions_size_limit {
					let direction_index = direction.index();
					directions_counter[direction_index] += 1;
					if directions_counter[direction_index] == directions_counter_modulus {
						directions_counter[direction_index] = 0;
						directions.push(direction.inverse().index());
					}
				}
				walkers[wi].add_element(direction, fib4[2] % levels, fib4[3] % max_radius);
				/*if (fib5[0] + wi) % (walkers.len() * 2) == 0 {
					walkers[wi].remove_first_element();
				}*/
			}
			*/
			let mut to_erase: Vec<usize> = Vec::new();
			for wi in 0..walkers.len() {
				let radius = walkers[wi].radius();
				let level = walkers[wi].level();
				//let level = 1;
				let level_change = 1;
				let mut can_draw = true;
				/*
				if (addresses[wi].row + radius) >= height {
					can_draw = false;
				}
				if (addresses[wi].row + height - radius) < height {
					can_draw = false;
				}
				if (addresses[wi].column + radius) >= width {
					can_draw = false;
				}
				if (addresses[wi].column + width - radius) < width {
					can_draw = false;
				}
				*/
				if can_draw {
					let neighborhood = image.torus.find_neighborhood(addresses[wi], radius, radius);
					for ni in 0..neighborhood.len() {
						if set_over_iterate[fib6[0] % set_over_iterate.len()] {
							image.address_set(neighborhood[ni], level);
						} else {
							let old_level = image.address_get(neighborhood[ni]);
							if increment[(fib5[0] + ni) % increment.len()] {
								let new_level = old_level + level;
								/*
								if new_level < levels {
									image.address_set(neighborhood[ni], new_level);
								} else {
									image.address_set(neighborhood[ni], levels - 1);
								}
								*/
								image.address_set(neighborhood[ni], new_level % levels);
							} else {
								let new_level = (old_level + levels - level);
								/*
								if new_level < levels {
									image.address_set(neighborhood[ni], 0);
								} else {
									image.address_set(neighborhood[ni], new_level % levels);
								}
								*/
								image.address_set(neighborhood[ni], new_level % levels);
							}
						}
					}
				}
				let direction = walkers[wi].direction();
				let mut address_iterator = addresses[wi];
				let mut must_update = true;
				'address_iterator_loop: for i in 0..directions_power_factor
					* ((fib6[0] % directions_power_variations) + directions_min_power)
				{
					address_iterator = image.torus.find_neighbor(address_iterator, direction);
					last_address = address_iterator;
					/*
					let option = image
						.torus
						.find_neighbor_no_torus(address_iterator, direction);
					match option {
						None => {
							must_update = false;
							to_erase.push(wi);
							let direction_index = direction.index();
							let direction_count = directions
								.iter()
								.filter(|&direction_filter| *direction_filter == direction_index)
								.count();
							if direction_count > 1 {
								let index = directions
									.iter()
									.position(|direction_filter| {
										*direction_filter == direction_index
									})
									.unwrap();
								directions.remove(index);
							} else {
								directions.push(direction.inverse().index());
							}
							break 'address_iterator_loop;
						}
						Some(address) => {
							address_iterator = address;
							last_address = address_iterator;
						}
					}
					*/
				}
				if must_update {
					checkpoints.push(last_address);
					walkers[wi].iterate();
				}
			}
			'erase: for i in 0..to_erase.len() {
				if walkers.len() > to_erase[i] {
					walkers.remove(to_erase[i]);
					addresses.remove(to_erase[i]);
				}
			}
			let mut length = checkpoints.len();
			while length > 64 {
				checkpoints.remove(0);
				length = checkpoints.len();
			}
			iterate_fibn(&mut fib2);
			iterate_fibn(&mut fib3);
			iterate_fibn(&mut fib4);
			iterate_fibn(&mut fib5);
			iterate_fibn(&mut fib6);
			//image.save_image(&folder_path, iteration);
			if iteration % iteration_modulus == 0 {
				println!("iteration {}", iteration / iteration_modulus);
				image.save_image(&folder_path, iteration / iteration_modulus);
			}
		}
	}
}

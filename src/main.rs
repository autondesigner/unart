#![allow(warnings)]

mod address;
mod address_book;
mod automaton;
mod auton_a1;
mod buildable;
mod cell;
mod color;
mod direction;
mod image_torus;
mod rule_key;
mod tape;
mod torus;
mod track;
mod util;
mod walker;
use crate::auton_a1::*;

fn main() {
	let child = std::thread::Builder::new()
		.stack_size(1024 * 1024 * 1024)
		.spawn(move || {
			AutonA1::render();
		})
		.unwrap();
	let res = child.join();
}

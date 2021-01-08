use crate::direction::*;
use crate::track::*;

pub struct WalkerElement {
	pub direction: Direction,
	pub level: usize,
	pub length: usize,
}
impl WalkerElement {
	fn new(direction: Direction, level: usize, length: usize) -> WalkerElement {
		WalkerElement {
			direction,
			level,
			length,
		}
	}
}

pub struct Walker {
	pub elements: Vec<WalkerElement>,
	pub index: usize,
}

impl Walker {
	pub fn new(direction: Direction, level: usize, length: usize) -> Walker {
		let mut elements = Vec::new();
		elements.push(WalkerElement::new(direction, level, length));
		let index = 0;
		Walker { elements, index }
	}
	pub fn add_element(&mut self, direction: Direction, level: usize, length: usize) {
		self.elements
			.push(WalkerElement::new(direction, level, length));
	}
	pub fn remove_first_element(&mut self) {
		if self.elements.len() > 1 {
			self.elements.remove(0);
			while self.index >= self.elements.len() {
				self.index -= 1;
			}
		}
	}
	pub fn length(&self) -> usize {
		self.elements[self.index].length
	}
	pub fn level(&self) -> usize {
		self.elements[self.index].level
	}
	pub fn direction(&self) -> Direction {
		self.elements[self.index].direction
	}
	pub fn iterate(&mut self) {
		self.index += 1;
		self.index %= self.elements.len();
	}
}

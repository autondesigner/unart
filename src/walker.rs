use crate::direction::*;
use crate::track::*;

pub struct WalkerElement {
	pub direction: Direction,
	pub level: usize,
	pub radius: usize,
}
impl WalkerElement {
	fn new(direction: Direction, level: usize, radius: usize) -> WalkerElement {
		WalkerElement {
			direction,
			level,
			radius,
		}
	}
}

pub struct Walker {
	pub elements: Vec<WalkerElement>,
	pub index: usize,
}

impl Walker {
	pub fn new(direction: Direction, level: usize, radius: usize) -> Walker {
		let mut elements = Vec::new();
		elements.push(WalkerElement::new(direction, level, radius));
		let index = 0;
		Walker { elements, index }
	}
	pub fn add_element(&mut self, direction: Direction, level: usize, radius: usize) {
		self.elements
			.push(WalkerElement::new(direction, level, radius));
	}
	pub fn remove_first_element(&mut self) {
		if self.elements.len() > 1 {
			self.elements.remove(0);
			while self.index >= self.elements.len() {
				self.index -= 1;
			}
		}
	}
	pub fn radius(&self) -> usize {
		self.elements[self.index].radius
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

mod device;

use std::path::Iter;
pub use device::Device;
use crate::request::Request;

#[derive(Debug)]
pub struct DeviceDeck {
	deck : Vec<Device>,
	current_index : usize
}

impl DeviceDeck {
	pub fn new(size : usize) -> Self {
		let mut deck = Vec::with_capacity(size);
		for i in 0..size {
			deck.push(Device::new(i));
		}
		Self { deck, current_index : 0 }
	}

	pub fn process_request(&mut self, request : Request) -> (usize, Option<Request>) {
		for _ in 0..self.deck.len() {
			if self.deck[self.current_index].is_free() {
				let temp_index = self.current_index;
				self.deck.get_mut(self.current_index ).unwrap().process(request);
				self.current_index = (self.current_index + 1) % self.deck.len();
				return (temp_index, None)
			}
			self.current_index = (self.current_index + 1) % self.deck.len();
		}
		(0, Some(request))
	}

	pub fn release_device(&mut self, num_of_device : usize) {
		self.deck[num_of_device].release()
	}

	pub fn len(&self) -> usize {
		self.deck.len()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Device> {
		self.deck.iter()
	}
}

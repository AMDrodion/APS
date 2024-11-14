use crate::special_event_api::SpecialEvent;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Buffer {
	deck : VecDeque<SpecialEvent>,
	max_size : usize,
	current_priority_of_source : usize
}

impl Buffer {
	pub fn new(size : usize) -> Self {
		Self { deck : VecDeque::with_capacity(size), max_size : size, current_priority_of_source : 0 }
	}

	pub fn is_full(&self) -> bool {
		self.deck.len() == self.max_size
	}

	pub fn push(&mut self, event : SpecialEvent) {
		if self.is_full() {
			let less_event = *self.deck
					.iter()
					.reduce(|l, r| {
						if l.event_type() < r.event_type() {
							l
						} else {
							r
						}
					})
					.unwrap_or(&event);
			if event > less_event {
				self.deck.retain(|&e| e != less_event);
			}
		}
		self.deck.push_back(event);
	}

}

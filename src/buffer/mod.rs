use crate::request::Request;

use std::collections::{BTreeSet};

#[derive(Debug)]
pub struct Buffer {
	deck : BTreeSet<Request>,
	max_size : usize,
	current_priority_of_source : usize
}

impl Buffer {
	pub fn new(size : usize) -> Self {
		Self { deck : BTreeSet::new(), max_size : size, current_priority_of_source : 0 }
	}

	pub fn is_full(&self) -> bool {
		self.deck.len() == self.max_size
	}

	pub fn push(&mut self, request: Request) {
		if self.is_full() {
			let less_request = self.deck.first().unwrap().clone();
			if request.source_num() < less_request.source_num() {
				self.deck.remove(&less_request);
			}
		}
		self.deck.insert(request);
	}

	pub fn get_request(&mut self) -> Request {
		let mut max_request= *self.deck.last().expect("Очередь пуста.\n");
		let temp_priority = max_request.source_num();
		for request in self.deck.iter().rev() {
			if request.source_num() == self.current_priority_of_source {
				max_request = *request;
				break
			}
		}
		self.deck.remove(&max_request);
		if temp_priority == max_request.source_num() {
			self.current_priority_of_source = temp_priority
		}
		max_request
	}
}

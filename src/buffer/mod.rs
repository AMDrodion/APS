use crate::request::Request;

use std::collections::{BTreeSet};
use std::fmt::{Display, Formatter};

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

	pub fn len(&self) -> usize {
		self.deck.len()
	}

	pub fn is_full(&self) -> bool {
		self.deck.len() >= self.max_size
	}

	pub fn push(&mut self, request: Request) -> Option<Request> {
		let mut deleted_request: Option<Request> = None;
		if self.is_full() {
			let less_request = self.deck.first().unwrap().clone();
			self.deck.remove(&less_request);
			deleted_request = Some(less_request);
		}
		self.deck.insert(request);
		deleted_request
	}

	pub fn get_request(&mut self) -> Option<Request> {
		if let Some(mut max_request) = self.deck.last().map(|r| *r) {
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
			Some(max_request)
		} else {
			None
		}
	}
}

impl Display for Buffer {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Буфер: \n\t Текущий приоритет: {} \n\t Содержимое: {:?}",
		       self.current_priority_of_source, self.deck)
	}
}

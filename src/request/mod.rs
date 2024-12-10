use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Request {
	source_num : usize,
	time : f64
}

impl Request {
	pub fn new(source_num: usize, time : f64) -> Self {
		Self { source_num, time }
	}

	pub fn source_num(&self) -> usize {
		self.source_num
	}

	pub fn time(&self) -> f64 {
		self.time
	}
}

impl Debug for Request {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}]", self.source_num)
	}
}

impl PartialOrd for Request {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let source_order = other.source_num.partial_cmp(&self.source_num);
		if source_order.unwrap() == Ordering::Equal {
			(other.time * 1_000_000_f64).round().partial_cmp(&(self.time * 1_000_000_f64).round())
		} else {
			source_order
		}
	}
}

impl Ord for Request {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialEq for Request{
	fn eq(&self, other: &Self) -> bool {
		(self.time * 1_000_000_f64).round() == (other.time * 1_000_000_f64).round() &&
				self.source_num == other.source_num
	}
}

impl Eq for Request {}

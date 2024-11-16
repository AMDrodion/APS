use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Request {
	source_num : usize,
	time : usize
}

impl Request {
	pub fn new(source_num: usize, time : usize) -> Self {
		Self { source_num, time }
	}

	pub fn source_num(&self) -> usize {
		self.source_num
	}

	pub fn time(&self) -> usize {
		self.time
	}
}

impl PartialOrd for Request {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let source_order = other.source_num.partial_cmp(&self.source_num);
		if source_order.unwrap() == Ordering::Equal {
			other.time.partial_cmp(&self.time)
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
		self.source_num == other.source_num && self.time == other.time
	}
}

impl Eq for Request {}

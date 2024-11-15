use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
		other.source_num.partial_cmp(&self.source_num)
	}
}

impl Ord for Request {
	fn cmp(&self, other: &Self) -> Ordering {
		other.source_num.cmp(&self.source_num())
	}
}

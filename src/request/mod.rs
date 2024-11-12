pub struct Request {
	source_num : usize,
	request_num : usize,
	time : usize
}

impl Request {
	pub fn new(source_num: usize, request_num: usize, time : usize) -> Self {
		Self { source_num, request_num, time }
	}

	pub fn source_num(&self) -> usize {
		self.source_num
	}

	pub fn request_num(&self) -> usize {
		self.request_num
	}

	pub fn time(&self) -> usize {
		self.time
	}
}
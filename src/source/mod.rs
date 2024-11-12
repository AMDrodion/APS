use crate::request::Request;

pub struct Source {
	num : usize,
	request_counter : usize,
	request_delay : usize
}

impl Source {
	pub fn new(num : usize, request_delay : usize) -> Self {
		Self { num, request_counter : 0, request_delay }
	}
	
	pub fn generate_request(&mut self) -> Request {
		self.request_counter += 1;
		Request::new(self.num, self.request_counter, self.request_delay * self.request_counter)
	}
}
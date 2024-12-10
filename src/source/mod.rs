use crate::request::Request;

#[derive(Debug)]
pub struct Source {
	num : usize,
	request_counter : usize,
	request_delay : f64
}

impl Source {
	pub fn new(num : usize, request_delay : f64) -> Self {
		Self { num, request_counter : 0, request_delay }
	}
	
	pub fn generate_request(&mut self) -> Request {
		self.request_counter += 1;
		Request::new(self.num, self.request_delay * self.request_counter as f64)
	}

	pub fn request_delay(&self) -> f64 {
		self.request_delay
	}

	pub fn request_counter(&self) -> usize {
		self.request_counter
	}
}
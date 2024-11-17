use crate::request::Request;

#[derive(Debug)]
pub struct Device {
	num : usize,
	is_free : bool
}

impl Device {
	pub fn new(num : usize) -> Self {
		Self { num, is_free: true }
	}

	pub fn num(&self) -> usize {
		self.num
	}

	pub fn is_free(&self) -> bool {
		self.is_free
	}

	pub fn process(&mut self, request: Request) {
		if !self.is_free {
			panic!("Прибор занят\n")
		}
		self.is_free = false
	}

	pub fn release(&mut self) {
		if self.is_free {
			panic!("Прибор уже освобожден\n")
		}
		self.is_free = true
	}
}

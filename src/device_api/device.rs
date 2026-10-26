use crate::request::Request;

#[derive(Debug)]
pub struct Device {
	is_free : bool
}

impl Device {
	pub fn new() -> Self {
		Self { is_free: true }
	}

	pub fn is_free(&self) -> bool {
		self.is_free
	}

	pub fn process(&mut self, _request: Request) {
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

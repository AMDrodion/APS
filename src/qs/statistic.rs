#[derive(Debug)]
pub struct Statistic {
	source_stat: Vec<(usize, usize)>, //сгенерированные / обработанные заявки
	device_stat: Vec<(usize, f64)>,   //обработанные заявки / время простоя
	buffer_stat: f64
}

impl Statistic {
	pub fn new(source_num: usize, device_num: usize) -> Self {
		let source_stat = vec![(0, 0); source_num];
		let device_stat = vec![(0, 0.0); device_num];

		Self {
			source_stat,
			device_stat,
			buffer_stat: 0.0,
		}
	}

	pub fn source_stat(&mut self, num: usize) -> &mut (usize, usize) {
		self.source_stat.get_mut(num).unwrap()
	}

	pub fn device_stat(&mut self, num: usize) -> &mut (usize, f64) {
		self.device_stat.get_mut(num).unwrap()
	}

	pub fn buffer_stat_mut(&mut self) -> &mut f64 {
		&mut self.buffer_stat
	}

	pub fn buffer_stat(&self) -> f64 {
		self.buffer_stat
	}

	pub fn source_stat_vec(&self) -> &Vec<(usize, usize)> {
		&self.source_stat
	}

	pub fn device_stat_vec(&self) -> &Vec<(usize, f64)> {
		&self.device_stat
	}

}
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
	sources : Vec<f64>,
	num_devices : usize,
	buffer_size : usize,
	processing_time: f64,
	simulation_time : f64
}

impl Config {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let config_text = fs::read_to_string("src/config/config.txt")?;
		let config : Config = toml::de::from_str(&config_text)?;
		Ok(config)
	}

	pub fn sources(&self) -> &Vec<f64> {
		&self.sources
	}

	pub fn num_devices(&self) -> usize {
		self.num_devices
	}

	pub fn buffer_size(&self) -> usize {
		self.buffer_size
	}

	pub fn simulation_time(&self) -> f64 {
		self.simulation_time
	}

	pub fn processing_time(&self) -> f64 {
		self.processing_time
	}
}

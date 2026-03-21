mod uml_events;
mod statistic;

use std::{fs, io};
use std::fs::OpenOptions;
use std::io::Write;
use rand::Rng;
use rand_distr::{Distribution, Exp};
use crate::buffer::Buffer;
use crate::device_api::DeviceDeck;
use crate::request::Request;
use crate::source::Source;
use crate::config::Config;
use crate::qs::statistic::Statistic;
use crate::qs::uml_events::UMLEvents;
use crate::special_event_api::{Event, SpecialEventType, SECalendar};

#[derive(Debug)]
pub struct QueuingSystem {
	buffer : Buffer,
	device_deck : DeviceDeck,
	sources : Vec<Source>,
	calendar : SECalendar,
	processing_time: f64,
	statistic: Statistic
}

impl QueuingSystem {
	pub fn new() -> Self {
		let mut calendar = SECalendar::new();
		let config = Config::new().unwrap();
		let buffer = Buffer::new(config.buffer_size());
		let device_deck = DeviceDeck::new(config.num_devices());
		let sources = config
				.sources()
				.into_iter()
				.enumerate()
				.map(|s| Source::new(s.0, *s.1))
				.collect::<Vec<Source>>();

		sources.iter()
				.enumerate()
				.for_each(
					|s| calendar.add_event(
							Event::new
									(SpecialEventType::NewRequest(s.0),
									 s.1.request_delay(),
									 false
									)
					)
				);
		calendar
				.add_event(Event::new(SpecialEventType::SimulationEnd, config.simulation_time(), false));
		Self { buffer, device_deck, sources, calendar,
			processing_time: config.processing_time(),
			statistic: Statistic::new(config.sources().len(), config.num_devices()) }
	}

	pub fn simulate(&mut self) {
		self.initialize_uml();
		let mut prev_buff_len = 0usize;
		let mut debug_mode = true;
		let mut num_of_new_requests = 0;
		let mut prev_event_time = 0f64;
		while !self.calendar.is_empty() {
			self.print_calendar();
			let event = self.calendar.get_event();
			self.update_statistic(event.time() - prev_event_time);
			prev_event_time = event.time();
			Self::debug(&mut debug_mode);
			match event.event_type() {
				SpecialEventType::DeviceRelease(num) => {
					num_of_new_requests = 0;
					self.release_devise(num, &event)
				}
				SpecialEventType::NewRequest(num) => {
					num_of_new_requests += 1;
					self.generate_request(num, &event)
					}
				_ => break
			}
			//self.update_uml(&event, num_of_new_requests, &mut prev_buff_len);
		}
		self.print_results(prev_event_time);
	}

	fn update_statistic(&mut self, time: f64) {
		self.device_deck
				.iter()
				.enumerate()
				.filter(|device| device.1.is_free())
				.for_each(|device| self.statistic.device_stat(device.0).1 += time);
		if self.buffer.is_full() {
			*self.statistic.buffer_stat_mut() += time;
		}
	}

	fn print_calendar(&self) {
		println!("{:10}|{:8}|{}", "Тип", "Время", "Признак");
		self.calendar.iter().rev().for_each(|event| {
			println!("{:10}|{:8.4}|0", event.event_type().to_string(), event.time())
		});
		self.device_deck
				.iter()
				.enumerate()
				.filter(|device| device.1.is_free())
				.for_each(|device| println!("Прибор_{:<3}|{:8.4}|1", device.0, 0));
		println!("\n{}\n", self.buffer);
		println!("**************\n")
	}

	fn print_results(&self, end_time: f64) {
		println!("Время моделирования: {:.1}\n", end_time);
		self.print_source_results();
		self.print_device_results(end_time);
		self.print_buffer_results(end_time);
	}

	fn print_buffer_results(&self, end_time: f64) {
		println!("\nБуфер\n\tЗаявки в буффере: {}\
							\n\tВремя в состоянии заполненности: {:.1}\
							\n\t% от общего времени: {:.1}",
		         self.buffer.len(),
		         self.statistic.buffer_stat(),
		         self.statistic.buffer_stat() / end_time * 100f64
		)
	}

	fn print_device_results(&self, end_time: f64) {
		println!("\nПриборы\n");
		println!("Номер | Обработано | Простаивание | % от общего времени");
		let mut percent_sum= 0f64;
		self.statistic
				.device_stat_vec()
				.iter()
				.enumerate()
				.for_each(|stat| {
					let percent = stat.1.1 / end_time * 100f64;
					percent_sum += percent;
					println!("{:6}|{:12}|{:14.3}|{:.1}",
					         stat.0, stat.1.0, stat.1.1,
					         stat.1.1 / end_time * 100f64
					)
				});
		println!("\nСредний % простаивания: {:.1}\n", percent_sum / self.device_deck.len() as f64);
	}

	fn print_source_results(&self) {
		println!("Источники\n");
		println!("Номер | Заявки | Отклонено | % отклонения");
		let mut percent_sum= 0f64;
		self.statistic
				.source_stat_vec()
				.iter()
				.enumerate()
				.for_each(
					|stat| {
						let percent = stat.1.1 as f64 / stat.1.0 as f64 * 100f64;
						percent_sum += percent;
						println!("{:6}|{:8}|{:11}|{:13.1}",
						         stat.0, stat.1.0, stat.1.1, percent
						)
					});
		println!("\nСредний % отказа: {:.1}\n", percent_sum / self.sources.len() as f64);
	}

	fn debug(debug_mode: &mut bool) {
		if *debug_mode {
			println!("Введите n для следующего шага. Для вывода результатов введите end");
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
			let command = input.trim();
			match command {
				"end" => {*debug_mode = false}
				_ => {}
			}
		}
	}

	fn generate_request(&mut self, num_of_source: usize, event: &Event) {
		self.statistic.source_stat(num_of_source).0 += 1;
		let device_result = self.device_deck
				.process_request(self.sources[num_of_source].generate_request());
		if let Some(req) = device_result.clone().1 {
			self.buffer.push(req).and_then(
				|req| {
					self.statistic.source_stat(req.source_num()).1 += 1;
					None::<Request>
				});
		} else {
			self.add_device_release_to_calendar(device_result.0, event);
			self.statistic.device_stat(device_result.0).0 += 1;
		}
		//self.write_event_to_uml(UMLEvents::Source(event.time(), num_of_source));
		self.calendar.add_event(Event::new(SpecialEventType::NewRequest(num_of_source),
		                                   event.time() + self.sources[num_of_source].request_delay(),
		                                   false))
	}

	fn release_devise(&mut self, num_to_release: usize, event: &Event) {
		self.device_deck.release_device(num_to_release);
		if let Some(req) = self.buffer.get_request() {
			let num_of_processing_device = self.device_deck.process_request(req).0;
			self.add_device_release_to_calendar(num_of_processing_device, event);
			self.statistic.device_stat(num_of_processing_device).0 += 1;
		}
	}

	fn add_device_release_to_calendar(&mut self, num: usize, event: &Event) {
		let lambda = 1.0 / self.processing_time;
		let exp_distribution = Exp::new(lambda).unwrap();
		let mut rng = rand::thread_rng();
		//self.write_event_to_uml(UMLEvents::Device(event.time() - event.time() * 0.00001, num, false));
		self.calendar.add_event(Event::new(
			SpecialEventType::DeviceRelease(num),
			event.time() + exp_distribution.sample(&mut rng),
			false)
		)
	}

	fn initialize_uml(&self) {
		let file_path = "diagram.puml";

		fs::write(file_path, "@startuml\n@enduml\n").unwrap();

		let mut content = fs::read_to_string(file_path).unwrap();

		for i in 0..self.sources.len() {
			content.insert_str
			(
				content.find("@enduml").unwrap(),
				format!("robust \"Источник{}\" as Src{}\n", i, i).as_str()
			)
		}

		content.insert_str(content.find("@enduml").unwrap(), "robust \"Буффер\" as Buff\n");

		for i in 0..self.device_deck.len() {
			content.insert_str
			(
				content.find("@enduml").unwrap(),
				format!("robust \"Прибор{}\" as Dev{}\n", i, i).as_str()
			)
		}

		content.insert_str(content.find("@enduml").unwrap(), "@0\n");

		for i in 0..self.sources.len() {
			content.insert_str(content.find("@enduml").unwrap(), format!("Src{} is false\n", i).as_str());
		}

		for i in 0..self.device_deck.len() {
			content.insert_str(content.find("@enduml").unwrap(), format!("Dev{} is false\n", i).as_str());
		}

		content.insert_str(content.find("@enduml").unwrap(),"Buff is 0\n");

		let mut file = OpenOptions::new()
				.write(true)
				.truncate(true)
				.open(file_path).unwrap();

		file.write_all(content.as_bytes()).unwrap();
	}

	fn write_event_to_uml(&self, event: UMLEvents) {
		let file_path = "diagram.puml";

		let mut content = fs::read_to_string(file_path).unwrap();

		match event {
			UMLEvents::Source(time, num) => {
				content.insert_str(content.find("@enduml").unwrap(), format!("@{}\n", time).as_str());
				content.insert_str(content.find("@enduml").unwrap(), format!("Src{} is {}\n", num, true).as_str());
				content.insert_str(content.find("@enduml").unwrap(), format!("@{}\n", time + 0.000000001).as_str());
				content.insert_str(content.find("@enduml").unwrap(), format!("Src{} is {}\n", num, false).as_str());
			}
			UMLEvents::Device(time, num, status) => {
				content.insert_str(content.find("@enduml").unwrap(), format!("@{}\n", time).as_str());
				content.insert_str(content.find("@enduml").unwrap(), format!("Dev{} is {}\n", num, status).as_str());
			}
			UMLEvents::Buffer(time, status) => {
				content.insert_str(content.find("@enduml").unwrap(), format!("@{}\n", time).as_str());
				content.insert_str(content.find("@enduml").unwrap(), format!("Buff is {}\n", status).as_str());
			}
		}

		let mut file = OpenOptions::new()
				.write(true)
				.truncate(true)
				.open(file_path).unwrap();

		file.write_all(content.as_bytes()).unwrap();
	}

	fn update_uml(&self, event: &Event, num_of_new_requests: usize, prev_buff_len: &mut usize) {
		for device in self.device_deck.iter() {
			self.write_event_to_uml(UMLEvents::Device(event.time() + 0.000000001 * num_of_new_requests as f64,
			                                          device.num(),
			                                          !device.is_free()))
		}
		let buff_len = self.buffer.len();
		if buff_len != *prev_buff_len {
			self.write_event_to_uml(UMLEvents::Buffer(event.time() + 0.0000001 * num_of_new_requests as f64, buff_len));
			*prev_buff_len = buff_len;
		}
	}
}
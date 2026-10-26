mod uml_events;
mod statistic;

use std::{fs, io};
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
	statistic: Statistic,
	uml: String,
	uml_device_states: Vec<bool>,
	uml_last_time: f64
}

const UML_FILE_PATH: &str = "diagram.puml";

const UML_DEVICE_GAP: f64 = 0.001;

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
			statistic: Statistic::new(config.sources().len(), config.num_devices()),
			uml: String::new(),
			uml_device_states: vec![false; config.num_devices()],
			uml_last_time: 0.0 }
	}

	pub fn simulate(&mut self) {
		self.initialize_uml();
		let mut prev_buff_len = 0usize;
		let mut debug_mode = true;
		let mut num_of_new_requests = 0;
		let mut prev_event_time = 0f64;
		while !self.calendar.is_empty() {
			if debug_mode {
				self.print_calendar();
			}
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
			self.update_uml(&event, num_of_new_requests, &mut prev_buff_len);
		}
		self.save_uml();
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
		self.write_event_to_uml(UMLEvents::Source(event.time(), num_of_source));
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
		
		if self.uml_device_states[num] {
			self.write_event_to_uml(UMLEvents::Device(event.time() - UML_DEVICE_GAP, num, false));
			self.uml_device_states[num] = false;
		}
		
		self.calendar.add_event(Event::new(
			SpecialEventType::DeviceRelease(num),
			event.time() + exp_distribution.sample(&mut rng),
			false)
		)
	}

	fn initialize_uml(&mut self) {
		self.uml.clear();
		self.uml_device_states.fill(false);
		self.uml_last_time = 0.0;
		self.uml.push_str("@startuml\n");

		for i in 0..self.sources.len() {
			self.uml.push_str(&format!("robust \"Источник{}\" as Src{}\n", i, i));
		}

		self.uml.push_str("robust \"Буффер\" as Buff\n");

		for i in 0..self.device_deck.len() {
			self.uml.push_str(&format!("robust \"Прибор{}\" as Dev{}\n", i, i));
		}

		self.uml.push_str("@0\n");

		for i in 0..self.sources.len() {
			self.uml.push_str(&format!("Src{} is false\n", i));
		}

		for i in 0..self.device_deck.len() {
			self.uml.push_str(&format!("Dev{} is false\n", i));
		}

		self.uml.push_str("Buff is 0\n");
	}
	
	fn write_uml_time(&mut self, time: f64) {
		let time = time.max(self.uml_last_time);
		self.uml_last_time = time;
		self.uml.push_str(&format!("@{}\n", time));
	}

	fn write_event_to_uml(&mut self, event: UMLEvents) {
		match event {
			UMLEvents::Source(time, num) => {
				self.write_uml_time(time);
				self.uml.push_str(&format!("Src{} is {}\n", num, true));
				self.write_uml_time(time + 0.000000001);
				self.uml.push_str(&format!("Src{} is {}\n", num, false));
			}
			UMLEvents::Device(time, num, status) => {
				self.write_uml_time(time);
				self.uml.push_str(&format!("Dev{} is {}\n", num, status));
			}
			UMLEvents::Buffer(time, status) => {
				self.write_uml_time(time);
				self.uml.push_str(&format!("Buff is {}\n", status));
			}
		}
	}

	fn update_uml(&mut self, event: &Event, num_of_new_requests: usize, prev_buff_len: &mut usize) {
		let device_states = self.device_deck
				.iter()
				.map(|device| !device.is_free())
				.collect::<Vec<bool>>();
		for (num, is_busy) in device_states.into_iter().enumerate() {
			if is_busy != self.uml_device_states[num] {
				self.write_event_to_uml(UMLEvents::Device(event.time() + 0.000000001 * num_of_new_requests as f64,
				                                          num,
				                                          is_busy));
				self.uml_device_states[num] = is_busy;
			}
		}
		let buff_len = self.buffer.len();
		if buff_len != *prev_buff_len {
			self.write_event_to_uml(UMLEvents::Buffer(event.time() + 0.0000001 * num_of_new_requests as f64, buff_len));
			*prev_buff_len = buff_len;
		}
	}

	fn save_uml(&mut self) {
		self.uml.push_str("@enduml\n");
		fs::write(UML_FILE_PATH, &self.uml).unwrap();
	}
}

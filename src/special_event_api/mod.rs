mod special_event_type;
mod special_event;

use std::collections::BinaryHeap;
pub use special_event_type::SpecialEventType;
pub use special_event::Event;

pub struct SECalendar {
	heap : BinaryHeap<Event>
}

impl SECalendar {
	pub fn new() -> Self {
		Self { heap : BinaryHeap::new() }
	}

	pub fn add_event(&mut self, se : Event) {
		self.heap.push(se)
	}

	pub fn get_event(&mut self) -> Event {
		self.heap.pop().expect("Календарь событий пуст\n")
	}
}

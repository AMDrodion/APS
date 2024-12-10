mod special_event_type;
mod special_event;

use std::collections::{BTreeSet};
pub use special_event_type::SpecialEventType;
pub use special_event::Event;

#[derive(Debug)]
pub struct SECalendar {
	heap : BTreeSet<Event>
}

impl SECalendar {
	pub fn new() -> Self {
		Self { heap : BTreeSet::new() }
	}

	pub fn add_event(&mut self, se : Event) {
		self.heap.insert(se);
	}

	pub fn get_event(&mut self) -> Event {
		self.heap.pop_last().expect("Календарь событий пуст\n")
	}

	pub fn is_empty(&self) -> bool {
		self.heap.is_empty()
	}

	pub fn iter(&self) -> std::collections::btree_set::Iter<'_, Event> {
		self.heap.iter()
	}
}

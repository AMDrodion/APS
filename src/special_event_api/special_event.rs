use std::cmp::Ordering;
use crate::special_event_api::SpecialEventType;

#[derive(Copy, Clone, Debug)]
pub struct Event {
	event_type : SpecialEventType,
	time : f64,
	mark : bool
}

impl Event {
	pub fn new(event_type: SpecialEventType, time: f64, mark: bool) -> Self {
		Self { event_type, time, mark }
	}

	pub fn event_type(&self) -> SpecialEventType {
		self.event_type
	}

	pub fn time(&self) -> f64 {
		self.time
	}

	pub fn mark(&self) -> bool {
		self.mark
	}
}

impl PartialEq for Event {
	fn eq(&self, other: &Self) -> bool {
		(self.time * 1_000_000_f64).round() == (other.time * 1_000_000_f64).round()
				&& self.event_type == other.event_type
	}
}

impl Eq for Event {}

impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let cmp = other.time().partial_cmp(&self.time());
		cmp.and_then(|ordering| {
			if ordering == Ordering::Equal {
				self.event_type().partial_cmp(&other.event_type())
			} else {
				Some(ordering)
			}
		})
	}
}

impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

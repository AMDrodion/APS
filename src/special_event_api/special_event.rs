use std::cmp::Ordering;
use crate::special_event_api::SpecialEventType;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Request {
	event_type : SpecialEventType,
	time : usize,
	mark : bool
}

impl Request {
	pub fn new(event_type: SpecialEventType, time: usize, mark: bool) -> Self {
		Self { event_type, time, mark }
	}

	pub fn event_type(&self) -> SpecialEventType {
		self.event_type
	}

	pub fn time(&self) -> usize {
		self.time
	}

	pub fn mark(&self) -> bool {
		self.mark
	}
}

impl PartialOrd for Request {
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

impl Ord for Request {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Ordering::Equal)
	}
}

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, Debug)]
pub enum SpecialEventType {
	DeviceRelease(usize),
	NewRequest(usize),
	SimulationEnd
}

impl Ord for SpecialEventType {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl PartialOrd for SpecialEventType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(_, SpecialEventType::SimulationEnd) => Some(Greater),
			(SpecialEventType::SimulationEnd, _) => Some(Less),
			(SpecialEventType::SimulationEnd, SpecialEventType::SimulationEnd) => Some(Equal),
			(SpecialEventType::DeviceRelease(n1), SpecialEventType::DeviceRelease(n2)) => {
				n2.partial_cmp(&n1)
			}
			(SpecialEventType::DeviceRelease(_), _) => Some(Greater),
			(_, SpecialEventType::DeviceRelease(_)) => Some(Less),
			(SpecialEventType::NewRequest(n1), SpecialEventType::NewRequest(n2)) => {
				n2.partial_cmp(n1)
			}
		}
	}
}

impl PartialEq for SpecialEventType {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(SpecialEventType::DeviceRelease(n1), SpecialEventType::DeviceRelease(n2)) => *n1 == *n2,
			(SpecialEventType::NewRequest(n1), SpecialEventType::NewRequest(n2)) => *n1 == *n2,
			(SpecialEventType::SimulationEnd, SpecialEventType::SimulationEnd) => true,
			_ => false
		}
	}
}

impl Display for SpecialEventType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SpecialEventType::NewRequest(n) => write!(f, "Источник_{}", n),
			SpecialEventType::DeviceRelease(n) => write!(f, "Прибор_{}", n),
			SpecialEventType::SimulationEnd => write!(f, "Конец")
		}
	}
}

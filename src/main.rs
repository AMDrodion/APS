mod request;
mod source;
mod special_event_api;
mod buffer;

use buffer::Buffer;
use special_event_api::{SpecialEvent, SpecialEventType, SECalendar};

fn main() {
    println!("Hello, world!");
    let a1 : SpecialEvent = SpecialEvent::new(SpecialEventType::NewRequest(0), 0, true);
    let a2 : SpecialEvent = SpecialEvent::new(SpecialEventType::NewRequest(6), 40, true);
    let a3 : SpecialEvent = SpecialEvent::new(SpecialEventType::NewRequest(4), 40, true);
    let a4 : SpecialEvent = SpecialEvent::new(SpecialEventType::NewRequest(3), 40, true);
    let mut b : Buffer = Buffer::new(3);
    b.push(a1);
    b.push(a2);
    b.push(a3);
    println!("{:?}", b);
    b.push(a4);
    println!("{:?}", b);
}

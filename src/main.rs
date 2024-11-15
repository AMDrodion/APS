mod request;
mod source;
mod special_event_api;
mod buffer;

use buffer::Buffer;
use special_event_api::{SpecialEventType, SECalendar};
use request::Request;

fn main() {
    println!("Hello, world!");
    let a1 : Request = Request::new(0, 0);
    let a2 : Request = Request::new(6, 40);
    let a3 : Request = Request::new(4, 40);
    let a4 : Request = Request::new(3, 40);
    let mut b : Buffer = Buffer::new(3);
    b.push(a1);
    b.push(a2);
    b.push(a3);
    println!("{:?}", b);
    b.push(a4);
    println!("{:?}", b);
}

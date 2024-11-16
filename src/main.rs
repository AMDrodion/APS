mod request;
mod source;
mod special_event_api;
mod buffer;
mod device_api;

use buffer::Buffer;
use special_event_api::{SpecialEventType, SECalendar};
use request::Request;
use device_api::DeviceDeck;

fn main() {
    let d = DeviceDeck::new(6);
    println!("{:?}", d);
    let a1 : Request = Request::new(2, 0);
    let a2 : Request = Request::new(6, 40);
    let a3 : Request = Request::new(4, 40);
    let a4 : Request = Request::new(3, 40);
    let a5 : Request = Request::new(3, 50);
    let mut b : Buffer = Buffer::new(4);
    b.push(a1);
    b.push(a2);
    b.push(a3);
    b.push(a4);
    println!("{:?}", b);
    b.push(a5);
    println!("{:?}", b);
    println!("{:?}", b.get_request());
    println!("{:?}", b.get_request());
    b.push(Request::new(2, 0));
    println!("{:?}", b.get_request());
    println!("{:?}", b.get_request())
}

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
    let mut d = DeviceDeck::new(4);
    println!("{:?}", d);
    let a1 : Request = Request::new(2, 0);
    let a2 : Request = Request::new(6, 40);
    let a3 : Request = Request::new(4, 40);
    let a4 : Request = Request::new(3, 40);
    let a5 : Request = Request::new(3, 50);
    let mut b : Buffer = Buffer::new(4);
    println!("{:?}", d.process_request(a1));
    println!("{:?}", d);
    println!("{:?}", d.process_request(a2));
    println!("{:?}", d);
    println!("{:?}", d.process_request(a3));
    println!("{:?}", d);
    println!("{:?}", d.process_request(a4));
    println!("{:?}", d);
    d.release_device(2);
    println!("{:?}", d);
    println!("{:?}", d.process_request(a5));
    println!("{:?}", d);
}

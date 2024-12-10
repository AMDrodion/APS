mod request;
mod source;
mod special_event_api;
mod buffer;
mod device_api;
mod config;
mod qs;

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use config::Config;
use buffer::Buffer;
use special_event_api::{SpecialEventType, SECalendar};
use request::Request;
use device_api::DeviceDeck;
use qs::QueuingSystem;

fn main() {
    let mut a = QueuingSystem::new();
    a.simulate()
}

mod request;
mod source;
mod special_event_api;
mod buffer;
mod device_api;
mod config;
mod qs;

use qs::QueuingSystem;

fn main() {
    let mut a = QueuingSystem::new();
    a.simulate()
}

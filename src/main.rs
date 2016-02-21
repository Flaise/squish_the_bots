#![deny(unused_must_use)]

extern crate rand;

#[macro_use]
mod macros;

mod action;
mod appearance;
mod area;
mod entity;
mod lobby;
mod network;
mod notification;
mod positioned;
mod pushable;
mod session;
mod space;

mod tests;

mod example_bots;


use std::time::Duration;
use std::thread;
use network::*;
use example_bots::hunter;

fn main() {
    let server = single_lobby("127.0.0.1:0", Duration::from_millis(50)).unwrap();
    
    let handle2 = hunter::start(server.addr, "Bot A".to_string(), true).unwrap();
    
    hunter::start(server.addr, "Bot B".to_string(), false).unwrap().join().unwrap();
}

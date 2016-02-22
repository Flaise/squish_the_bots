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
use network::*;
use example_bots::hunter;

fn main() {
    let server = single_lobby("127.0.0.1:0", Duration::from_millis(50),
                              Duration::from_millis(450)).unwrap();
    
    println!("Waiting for connections to {:?}", server.addr);
    
    server.join().unwrap().unwrap();
}

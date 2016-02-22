#![deny(unused_must_use)]

extern crate rand;

#[macro_use]
mod macros;

mod action;
mod appearance;
mod area;
mod cooldown;
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


use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use network::*;
use example_bots::hunter;

fn main() {
    let port = match env::args().nth(1) {
        None => "0".to_string(),
        Some(arg) => arg,
    };
    
    let address: SocketAddr = match FromStr::from_str(&("127.0.0.1:".to_string() + &port)) {
        Err(AddrParseError(..)) => panic!("Invalid port."),
        Ok(address) => address,
    };
    
    let server = single_lobby(address, Duration::from_millis(50),
                              Duration::from_millis(450)).unwrap();
    
    println!("Waiting for connections to {:?}", server.addr);
    
    server.join().unwrap().unwrap();
}

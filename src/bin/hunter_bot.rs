extern crate squish_the_bots;

use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

use squish_the_bots::example_bots::hunter;


fn main() {
    let command = env::args().nth(0).unwrap();
    let arg = match env::args().nth(1) {
        None => panic!("Usage: {} <address>", command),
        Some(arg) => arg,
    };
    
    let address: SocketAddr = match arg.to_socket_addrs() {
        Err(..) => panic!("Invalid address. Expecting format #.#.#.#:# or domain:#"),
        Ok(mut iterator) => iterator.next().unwrap(),
    };
    
    hunter::run(address, "Hunter Bot".to_string(), true);
}

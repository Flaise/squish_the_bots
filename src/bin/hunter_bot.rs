extern crate squish_the_bots;

use std::env;
use std::net::SocketAddr;

use squish_the_bots::example_bots::hunter;


fn main() {
    let command = env::args().nth(0).unwrap();
    let arg = match env::args().nth(1) {
        None => panic!("Usage: {} IP:PORT", command),
        Some(arg) => arg,
    };
    
    let address: SocketAddr = match arg.parse() {
        Err(..) => panic!("Invalid address \"{}\". Usage: {} IP:PORT", arg, command),
        Ok(address) => address,
    };
    
    hunter::run(address, "Hunter Bot".to_string(), true);
}

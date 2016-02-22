extern crate squish_the_bots;

use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;

use squish_the_bots::example_bots::hunter;


fn main() {
    let command = env::args().nth(0).unwrap();
    let arg = match env::args().nth(1) {
        None => panic!("Usage: {} <address>", command),
        Some(arg) => arg,
    };
    
    let address: SocketAddr = match FromStr::from_str(arg.as_ref()) {
        Err(AddrParseError(..)) => panic!("Invalid address."),
        Ok(address) => address,
    };
    
    hunter::run(address, "Hunter Bot".to_string(), true);
}

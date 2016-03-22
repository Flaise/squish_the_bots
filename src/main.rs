#![deny(unused_must_use)]

extern crate rand;
extern crate getopts;

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
mod vector;

mod tests;
mod example_bots;


use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use network::*;
use getopts::Options;


fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let args = args.collect::<Vec<_>>();
    
    let mut options = Options::new();
    options.reqopt("o", "static-directory", "Directory to output static files to.", "DIR");
    options.reqopt("s", "simulation", "Port to accept bot socket connections on", "PORT");
    options.optopt("e", "simulation-external",
                   "External port of simulation, if different than internal.", "PORT");
    
    let matches = match options.parse(&args) {
        Ok(result) => result,
        Err(_) => {
            println!("{}", options.usage(&*options.short_usage(&*program)));
            return;
        }
    };
    
    let sim_port = matches.opt_str("simulation").unwrap();
    let sim_address: SocketAddr =
        match FromStr::from_str(&("0.0.0.0:".to_string() + &sim_port)) {
            Err(AddrParseError(..)) => panic!("Invalid simulation port."),
            Ok(address) => address,
        };
    
    let simulation = single_lobby(sim_address, Duration::from_millis(2000),
                                  Duration::from_millis(450)).unwrap();
    println!("Waiting for simulation socket connections on {}", simulation.addr);
    
    
    let external_port = matches.opt_str("simulation-external")
                               .unwrap_or(simulation.addr.port().to_string());
    
    let static_dir = matches.opt_str("static-directory").unwrap();
    write_statics(&static_dir, &external_port);
    
    simulation.join().unwrap().unwrap();
}

fn write_statics(static_dir: &String, external_port: &str) {
    let index_page = include_str!("./index.html");
    let index_page = index_page.replace("#####", &external_port);
    
    let mut f = File::create(static_dir.to_string() + "/index.html").unwrap();
    f.write_all(index_page.as_bytes()).unwrap();
    
    let mut f = File::create(static_dir.to_string() + "/demo.gif").unwrap();
    f.write_all(include_bytes!("./demo.gif")).unwrap();
}

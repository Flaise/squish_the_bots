#![deny(unused_must_use)]

extern crate rand;
extern crate hyper;
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

mod tests;

mod example_bots;


use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use network::*;
use getopts::Options;
use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};


fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let args = args.collect::<Vec<_>>();
    
    let mut options = Options::new();
    options.reqopt("w", "web", "Port to accept HTTP requests on", "PORT");
    options.reqopt("s", "simulation", "Port to accept bot socket connections on", "PORT");
    
    let matches = match options.parse(&args) {
        Ok(result) => result,
        Err(_) => {
            println!("{}", options.usage(&*options.short_usage(&*program)));
            return;
        }
    };
    
    let web_port = matches.opt_str("web").unwrap();
    let sim_port = matches.opt_str("simulation").unwrap();
    
    let web_address: SocketAddr =
        match FromStr::from_str(&("127.0.0.1:".to_string() + &web_port)) {
            Err(AddrParseError(..)) => panic!("Invalid web port."),
            Ok(address) => address,
        };
    
    let sim_address: SocketAddr =
        match FromStr::from_str(&("127.0.0.1:".to_string() + &sim_port)) {
            Err(AddrParseError(..)) => panic!("Invalid simulation port."),
            Ok(address) => address,
        };
    
    let simulation = single_lobby(sim_address, Duration::from_millis(2000),
                                  Duration::from_millis(450)).unwrap();
    println!("Waiting for simulation socket connections on {}", simulation.addr);
    
    let index_page = include_str!("./index.html");
    let index_page = index_page.replace("#####", simulation.addr.port().to_string().as_ref());
    
    let web_server = hyper::Server::http(web_address).unwrap()
        .handle(move|req: Request, res: Response| {
            handler(req, res, &index_page)
        }).unwrap();
    println!("Waiting for HTTP requests on {}", web_server.socket);
    
    simulation.join().unwrap().unwrap();
}

// Rust doesn't allow types with destructors as constants.
// See https://github.com/rust-lang/rfcs/issues/913
// const text_html: ContentType = ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]));

fn handler(_: Request, mut res: Response, index_page: &str) {
    {
        let mut status = res.status_mut();
        *status = StatusCode::Ok;
    }
    res.headers_mut().set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    res.send(index_page.as_ref()).unwrap();
}

use std::net::*;
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::time::Duration;
use entity::*;
use space::*;
use space::Direction::*;
    

fn extract_connections(tcp_listener: TcpListener, sender: Sender<(TcpStream, SocketAddr)>)
        -> io::Result<thread::JoinHandle<io::Result<()>>> {
    thread::Builder::new().name("TCP listener".to_string()).spawn(move|| {
        loop {
            let connection = try!(tcp_listener.accept());
                        
            match sender.send(connection) {
                Ok(()) => continue,
                Err(SendError(_)) => return Ok(()),
            }
        }
    })
}


#[test]
fn terminate() {
    let (sender, receiver) = channel();
    
    let tcp_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = tcp_listener.local_addr().unwrap();
    println!("listening to address {:?}", addr);
    
    let handle = extract_connections(tcp_listener, sender).unwrap();
    
    drop(receiver);
    
    drop(TcpStream::connect(addr)); // workaround for accept() function not timing out
    
    
    handle.join().unwrap();
}

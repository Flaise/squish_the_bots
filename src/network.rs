use std::io::{self, Read, Write};
use std::mem;
use std::net::*;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use entity::*;
use space::*;
use space::Direction::*;


struct Server {
    receiver: Option<Receiver<(TcpStream, SocketAddr)>>,
    addr: SocketAddr,
    join_handle: Option<JoinHandle<io::Result<()>>>,
}
impl Server {
    fn new<A: ToSocketAddrs>(address: A) -> io::Result<Server> {
        let tcp_listener = try!(TcpListener::bind(address));
        let local_addr = try!(tcp_listener.local_addr());
        
        let (sender, receiver) = channel();
        let join_handle = try!(extract_connections(tcp_listener, sender));
        
        Ok(Server {
            receiver: Some(receiver),
            addr: local_addr,
            join_handle: Some(join_handle)
        })
    }
    
    fn stop(mut self) -> JoinHandle<io::Result<()>> {
        mem::replace(&mut self.join_handle, None).unwrap()
        // implicitly calls #drop() to stop server thread
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        // must be called before the dummy connection is made
        drop(mem::replace(&mut self.receiver, None));
        
        // workaround for accept() function not timing out
        drop(TcpStream::connect(self.addr));
    }
}


fn extract_connections(tcp_listener: TcpListener, sender: Sender<(TcpStream, SocketAddr)>)
        -> io::Result<JoinHandle<io::Result<()>>> {
    thread::Builder::new().name("TCP Listener".to_string()).spawn(move|| {
        loop {
            let connection = try!(tcp_listener.accept());
            
            if sender.send(connection).is_err() {
                // Err means other thread hung up and is done with this TcpListener
                return Ok(())
            }
        }
    })
}


#[test]
fn terminate_explicit() {
    let server = Server::new("127.0.0.1:0").unwrap();
    let addr = server.addr;
    server.stop().join().unwrap().unwrap();
    
    assert!(TcpStream::connect(addr).is_err());
}

#[test]
fn terminate_implicit() {
    let (addr, join_handle) = {
        let mut server = Server::new("127.0.0.1:0").unwrap();
        let join_handle = mem::replace(&mut server.join_handle, None).unwrap();
        (server.addr, join_handle)
    };
    
    join_handle.join().unwrap().unwrap();
    
    assert!(TcpStream::connect(addr).is_err());
}

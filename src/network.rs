use std::io::{self, Read, Write, BufReader};
use std::mem;
use std::net::*;
use std::time::Duration;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Sender, Receiver, channel, SendError, RecvError};
use entity::*;
use space::*;
use space::Direction::*;
use lobby::*;


struct Server {
    addr: SocketAddr,
    join_handle: Option<JoinHandle<io::Result<()>>>,
    stopped: Arc<RwLock<bool>>,
}
impl Server {
    fn new<A, B>(address: A, callback: B)
            -> io::Result<Server>
            where A: ToSocketAddrs, B: 'static+Send+FnMut(TcpStream, SocketAddr) -> io::Result<()> {
        let tcp_listener = try!(TcpListener::bind(address));
        let local_addr = try!(tcp_listener.local_addr());
        
        let stopped = Arc::new(RwLock::new(false));
        
        let join_handle = try!(start(tcp_listener, callback, stopped.clone()));
        
        Ok(Server {
            addr: local_addr,
            join_handle: Some(join_handle),
            stopped: stopped,
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
        match self.stopped.write() {
            Err(_) => debug_unreachable!(), // readers do not poison the lock
            Ok(mut value) => *value = true,
        }
        
        // workaround for accept() function not timing out
        drop(TcpStream::connect(self.addr));
    }
}

fn start<A>(tcp_listener: TcpListener, mut callback: A, stopped: Arc<RwLock<bool>>)
        -> io::Result<JoinHandle<io::Result<()>>>
        where A: 'static+Send+FnMut(TcpStream, SocketAddr) -> io::Result<()> {
    thread::Builder::new().name("TCP Listener".to_string()).spawn(move|| {
        loop {
            let (stream, address) = try!(tcp_listener.accept());
            
            match stopped.read() {
                Err(_) => debug_unreachable!(return),
                Ok(value) => {
                    if *value {
                        return Ok(());
                    }
                }
            }
            
            try!(callback(stream, address));
        }
    })
}

fn single_lobby<A: ToSocketAddrs>(address: A, timeout: Duration) -> io::Result<Server> {
    let mut lobby = try!(Lobby::new());
    
    Server::new(address, move|stream: TcpStream, address: SocketAddr| {
        try!(stream.set_read_timeout(Some(timeout)));
        
        let stream2 = try!(stream.try_clone());
        
        match lobby.add(Participant::new_boxed(stream, stream2)) {
            Ok(()) => (),
            Err(SendError(participant)) => {
                // Lobby ended because it was empty
                
                lobby = try!(Lobby::new());
                if lobby.add(participant).is_err() {
                    // Newly created lobby shouldn't end until after first connection
                    
                    return Err(io::Error::new(io::ErrorKind::Other,
                                              "New lobby did not accept connection."));
                }
            }
        }
        Ok(())
    })
}


#[test]
fn terminate_explicit() {
    let callback = |_, _| panic!();
    let server = Server::new("127.0.0.1:0", callback).unwrap();
    let addr = server.addr;
    match server.stop().join() {
        Ok(Ok(())) => (),
        Ok(Err(a)) => panic!("{:?}", a),
        Err(a) => panic!("{:?}", a),
    }
    
    assert!(TcpStream::connect(addr).is_err());
}

#[test]
fn terminate_implicit() {
    let (addr, join_handle) = {
        let callback = |_, _| panic!();
        let mut server = Server::new("127.0.0.1:0", callback).unwrap();
        let join_handle = mem::replace(&mut server.join_handle, None).unwrap();
        (server.addr, join_handle)
    };
    
    join_handle.join().unwrap().unwrap();
    
    assert!(TcpStream::connect(addr).is_err());
}

#[test]
fn simple_interaction() {
    let server = single_lobby("127.0.0.1:0", Duration::from_millis(99999)).unwrap();
    {
        let addr = server.addr;
        
        let mut client_a = TcpStream::connect(addr).unwrap();
        
        thread::sleep(Duration::from_millis(100));
        
        client_a.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        
        let mut buf = [0; 10];
        assert!(client_a.read(&mut buf).is_err());
        
        let mut client_b = TcpStream::connect(addr).unwrap();
        
        client_a.write_all(&[1, 0, 0]).unwrap();
        client_b.write_all(&[1, 0, 0]).unwrap();
        
        // LookAt, 0 dx, 0 dy = 1 0 0
        
        thread::sleep(Duration::from_millis(100));
        
        let mut buf = [0; 10];
        let res = client_a.read(&mut buf);
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(res.unwrap(), 5);
        assert_eq!(buf[0..5], [5, 1, 6, 1, 1]);
        
        let res = client_b.read(&mut buf);
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(res.unwrap(), 4);
        assert_eq!(buf[0..4], [5, 1, 6, 1]);
        
        // NewRound, YourTurn, YouSee, Bot, YourTurn = 5 1 6 1 1
    }
    
    server.stop().join().unwrap().unwrap();
}

#[test]
fn client_timeout() {
    let server = single_lobby("127.0.0.1:0", Duration::from_millis(50)).unwrap();
    {
        let addr = server.addr;
        
        let mut client_a = TcpStream::connect(addr).unwrap();
        let mut client_b = TcpStream::connect(addr).unwrap();
        
        client_b.write_all(&[1, 0, 0, 1, 0, 0]).unwrap();
        
        // LookAt, 0 dx, 0 dy = 1 0 0
        
        thread::sleep(Duration::from_millis(100));
        
        let mut buf = [0; 10];
        let res = client_a.read(&mut buf);
        assert!(res.is_ok(), "{:?}", res);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf[0..2], [5, 1]);
        
        let mut buf = [200; 10];
        let res = client_b.read(&mut buf);
        assert!(res.is_ok(), "{:?}", res);
        let res = res.unwrap();
        assert!(res == 4, "{:?}", buf);
        assert_eq!(buf[0..4], [5, 1, 6, 1]);
        
        // NewRound, YourTurn, YouSee, Bot = 5 1 6 1
    }
    
    server.stop().join().unwrap().unwrap();
}

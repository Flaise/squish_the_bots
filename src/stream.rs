use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io;
use std::io::{Read, Write};
use std::thread;
use entity::*;
use space::*;
use space::Direction::*;


// fn
// TcpListener::bind("127.0.0.1:80")


// struct Server {
//     tcp_listener: TcpListener,
//     // area: Area,
// }
// // impl Server {
    
//     fn start<A: ToSocketAddrs>(addr: A) -> io::Result<Server> {
//         let listener = try!(TcpListener::bind(addr));
//         Ok(Server {
//             tcp_listener: listener,
//             // area: Area::new(),
//         })
//     }
// // }


fn run_one(tcp_listener: TcpListener) -> io::Result<thread::JoinHandle<()>> {
    thread::Builder::new().name("TCP listener".to_string()).spawn(move|| {
        match tcp_listener.accept() {
            Err(error) => println!("{:?}", error),
            Ok((tcp_stream, addr)) => {
                
            },
        }
        // io::stdout().flush().unwrap();
    })
}



// #[test]
// fn connection() {
//     // let server = start("127.0.0.1:0").unwrap();
//     // let addr = server.tcp_listener.local_addr().unwrap();
//     //
//     // let mut stream = TcpStream::connect(addr).unwrap();
//
//
//     let area = Area::new(Rectangle::wh(North * 10 + East * 10));
//
//     let tcp_listener = try!(TcpListener::bind("127.0.0.1:0"));
//     let addr = tcp_listener.local_addr().unwrap();
//     run(tcp_listener, 1);
//
//     let mut stream = TcpStream::connect(addr).unwrap();
//
//     let mut buf = &[0, 0];
//     assert_eq!(stream.read(&mut buf), Ok(1));
//     assert_eq!(buf[0..0], &[1]);
//
//     // assert_eq!(area.count_type(EntityType::Bot), 1);
//
//
//     // let mut stream = TcpStream::connect(addr).unwrap();
// }

#[test]
fn connection() {
    let tcp_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = tcp_listener.local_addr().unwrap();
    println!("listening to address {:?}", addr);
    
    let handle = run_one(tcp_listener).unwrap();
    
    let mut stream = TcpStream::connect(addr).unwrap();
    
    let mut buf = [0, 0];
    // assert_eq!(stream.read(&mut buf).ok(), Some(1));
    // assert_eq!(&buf[0..0], &[1]);
    
    handle.join().unwrap();
}

use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io;
use entity::*;
use space::*;
use space::Direction::*;


// fn
// TcpListener::bind("127.0.0.1:80")


struct Server {
    tcp_listener: TcpListener,
    area: Area,
}
// impl Server {
    
    fn start<A: ToSocketAddrs>(addr: A) -> io::Result<Server> {
        let listener = try!(TcpListener::bind(addr));
        Ok(Server {
            tcp_listener: listener,
            area: Area::new(),
        })
    }
// }





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

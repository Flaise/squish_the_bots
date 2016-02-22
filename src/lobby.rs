use std::io::{self, Read, Write, Cursor};
use std::sync::mpsc::*;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use entity::*;
use area::*;
use notification::*;
use session::*;


pub struct Participant {
    input: Box<Read>,
    output: Box<Write>,
}
impl Participant {
    pub fn new(input: Box<Read>, output: Box<Write>) -> Participant {
        Participant {
            input: input,
            output: output,
        }
    }
    pub fn new_boxed<R: 'static+Read+Sized, W: 'static+Write+Sized>(input: R, output: W)
            -> Participant {
        Self::new(Box::new(input), Box::new(output))
    }
}
unsafe impl Send for Participant {
}


pub struct Lobby {
    sender: Sender<Participant>,
    join_handle: JoinHandle<()>,
}
impl Lobby {
    pub fn new(delay: Duration) -> io::Result<Lobby> {
        let (sender, receiver) = channel();
        let join_handle = try!(start_lobby(receiver, delay));
        
        Ok(Lobby {
            sender: sender,
            join_handle: join_handle,
        })
    }
    
    pub fn add(&self, participant: Participant) -> Result<(), SendError<Participant>> {
        self.sender.send(participant)
    }
    
    fn stop(self) -> JoinHandle<()> {
        self.join_handle
    }
}


fn start_lobby(receiver: Receiver<Participant>, delay: Duration) -> io::Result<JoinHandle<()>> {
    thread::Builder::new().name("Instance Runner".to_string()).spawn(move|| {
        let mut participants = vec![];
        
        loop {
            while participants.len() < 2 {
                match receiver.recv() {
                    Ok(participant) => participants.push((participant.input, participant.output)),
                    Err(RecvError) => return,
                }
            }
            
            loop {
                match receiver.try_recv() {
                    Ok(participant) => participants.push((participant.input, participant.output)),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }
            
            let elements = participants.drain(..).collect::<Vec<(Box<Read>, Box<Write>)>>();
            for (mut input, mut output) in elements {
                let notify_result = notify(&mut output, Notification::NewRound);
                match notify_result {
                    Ok(()) => participants.push((input, output)),
                    Err(error) => println!("Participant disconnected - {:?}", error),
                }
            }
            
            match participants.len() {
                0 => return,
                1 => continue,
                _ => execute_round(&mut participants, delay),
            }
        }
    })
}

#[cfg(test)]
use tests::{SharedWrite};
#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use std::rc::Rc;

#[test]
fn terminates_when_dropped() {
    let lobby = Lobby::new(Duration::from_millis(0)).unwrap();
    lobby.stop().join().unwrap();
}

#[test]
fn waits_for_2_participants() {
    let lobby = Lobby::new(Duration::from_millis(0)).unwrap();
    
    let output_a = Rc::new(RefCell::new(Vec::<u8>::new()));
    let output_b = Rc::new(RefCell::new(Vec::<u8>::new()));
    
    let shared_a = SharedWrite::new(output_a.clone());
    let shared_b = SharedWrite::new(output_b.clone());
    
    let a = Participant::new_boxed(Cursor::new(vec![]), shared_a);
    let b = Participant::new_boxed(Cursor::new(vec![]), shared_b);
    
    lobby.add(a).unwrap();
    
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(*output_a.borrow(), vec![]);
    assert_eq!(*output_b.borrow(), vec![]);
    
    lobby.add(b).unwrap();
    
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(*output_a.borrow(), vec![5, 1]);
    assert_eq!(*output_b.borrow(), vec![5, 1]);
    
    lobby.stop().join().unwrap();
}

#[test]
fn waits_after_disconnection() {
    let lobby = Lobby::new(Duration::from_millis(0)).unwrap();
    
    let output_a = Rc::new(RefCell::new(Vec::<u8>::new()));
    let mut shared_a = SharedWrite::new(output_a.clone());
    drop(output_a);
    shared_a.close();
    let a = Participant::new_boxed(Cursor::new(vec![]), shared_a);
    
    let output_b = Rc::new(RefCell::new(Vec::<u8>::new()));
    let shared_b = SharedWrite::new(output_b.clone());
    let b = Participant::new_boxed(Cursor::new(vec![]), shared_b);
    
    lobby.add(a).unwrap();
    
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(*output_b.borrow(), vec![]);
    
    lobby.add(b).unwrap();
    
    thread::sleep(Duration::from_millis(100));
    
    assert_eq!(*output_b.borrow(), vec![5]);
    
    lobby.stop().join().unwrap();
}

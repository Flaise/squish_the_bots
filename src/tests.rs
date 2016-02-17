#![cfg(test)]

use std::io::{self, Write, Cursor, ErrorKind};
use std::rc::Rc;
use std::cell::RefCell;


pub struct SharedWrite {
    pub writer: Rc<RefCell<Write>>,
    pub closed: bool,
}
impl SharedWrite {
    pub fn new(writer: Rc<RefCell<Write>>) -> SharedWrite {
        SharedWrite {
            writer: writer,
            closed: false,
        }
    }
    pub fn close(&mut self) {
        self.closed = true
    }
}
impl Write for SharedWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.closed {
            return Err(io::Error::new(ErrorKind::Other, "Stream is closed."));
        }
        self.writer.borrow_mut().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.closed {
            return Err(io::Error::new(ErrorKind::Other, "Stream is closed."));
        }
        self.writer.borrow_mut().flush()
    }
}

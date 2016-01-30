use std::error::Error;
use std::fmt;

use space::*;


#[derive(Debug)]
pub struct NotFoundError;
impl Error for NotFoundError {
    fn description(&self) -> &str {
        "No bot found at the specified location."
    }
}
impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}


pub struct Bots {
    instances: Vec<Position>
}
impl Bots {
    fn new() -> Bots {
        Bots {
            instances: Vec::new()
        }
    }
    
    fn make(&mut self, position: Position) {
        self.instances.push(position);
    }
    
    fn occupied(&self, position: Position) -> bool {
        self.instances.contains(&position)
    }
    
    fn shift(&mut self, from: Position, to: Position) -> Result<(), NotFoundError> {
        for e in self.instances.iter_mut() {
            if e == &from {
                *e = to;
                return Ok(())
            }
        }
        Err(NotFoundError)
    }
}


#[test]
fn instantiation() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    let c = Position::new(-1, 3);
    
    let mut bots = Bots::new();
    assert!(!bots.occupied(a));
    assert!(!bots.occupied(b));
    assert!(!bots.occupied(c));
    
    bots.make(a);
    assert!(bots.occupied(a));
    assert!(!bots.occupied(b));
    assert!(!bots.occupied(c));
    
    bots.make(b);
    assert!(bots.occupied(a));
    assert!(bots.occupied(b));
    assert!(!bots.occupied(c));
}

#[test]
fn movement() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    
    let mut bots = Bots::new();
    
    bots.make(a);
    assert!(bots.occupied(a));
    assert!(!bots.occupied(b));
    
    bots.shift(a, b).unwrap();
    assert!(!bots.occupied(a));
    assert!(bots.occupied(b));
}

#[test]
fn movement_nonexistent() {
    let mut bots = Bots::new();
    
    match bots.shift(Position::new(0, 1), Position::new(2, 5)) {
        Ok(_) => panic!(),
        Err(_) => ()
    }
}

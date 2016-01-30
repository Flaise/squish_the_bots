use std::error::Error;

use space::*;


#[derive(Debug)]
pub enum BotError {
    NotFound,
    AlreadyOccupied,
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
    
    fn make(&mut self, position: Position) -> Result<(), BotError> {
        if self.occupied(position) {
            return Err(BotError::AlreadyOccupied);
        }
        self.instances.push(position);
        Ok(())
    }
    
    fn occupied(&self, position: Position) -> bool {
        self.instances.contains(&position)
    }
    
    // TODO?: -> Result2<(), AlreadyOccupiedError, NotFoundError>
    fn shift(&mut self, from: Position, to: Position) -> Result<(), BotError> {
        if self.occupied(to) {
            return Err(BotError::AlreadyOccupied);
        }
        for e in self.instances.iter_mut() {
            if e == &from {
                *e = to;
                return Ok(())
            }
        }
        Err(BotError::NotFound)
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
    
    bots.make(a).unwrap();
    assert!(bots.occupied(a));
    assert!(!bots.occupied(b));
    assert!(!bots.occupied(c));
    
    bots.make(b).unwrap();
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
        Err(BotError::NotFound) => (),
        _ => panic!(),
    }
}

#[test]
fn already_occupied() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    let mut bots = Bots::new();
    
    bots.make(a).unwrap();
    bots.make(b).unwrap();
    
    match bots.make(a) {
        Err(BotError::AlreadyOccupied) => (),
        _ => panic!(),
    }
    assert!(bots.occupied(a));
    assert!(bots.occupied(b));
    
    match bots.shift(a, b) {
        Err(BotError::AlreadyOccupied) => (),
        _ => panic!(),
    }
    assert!(bots.occupied(a));
    assert!(bots.occupied(b));
}

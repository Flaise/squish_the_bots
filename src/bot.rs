use space::*;


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
        for e in &self.instances {
            if e == &position {
                return true
            }
        }
        false
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
    
    bots.make(Position::new(0, 0));
    assert!(bots.occupied(a));
    assert!(!bots.occupied(b));
    assert!(!bots.occupied(c));
    
    bots.make(Position::new(2, 0));
    assert!(bots.occupied(a));
    assert!(bots.occupied(b));
    assert!(!bots.occupied(c));
}

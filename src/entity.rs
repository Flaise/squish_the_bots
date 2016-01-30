use space::*;


#[derive(Debug)]
pub enum EntityError {
    NotFound,
    AlreadyOccupied,
}


struct Entity<D> {
    position: Position,
    tag: D,
}
impl<D> Entity<D> {
    fn new(position: Position, tag: D) -> Entity<D> {
        Entity {
            position: position,
            tag: tag
        }
    }
}


pub struct Entities<D> {
    instances: Vec<Entity<D>>
}
impl<D> Entities<D> {
    fn new() -> Entities<D> {
        Entities {
            instances: Vec::new()
        }
    }
    
    fn add(&mut self, instance: Entity<D>) -> Result<(), EntityError> {
        if self.occupied(instance.position) {
            return Err(EntityError::AlreadyOccupied);
        }
        self.instances.push(instance);
        Ok(())
    }
    
    fn occupied(&self, position: Position) -> bool {
        for inst in &self.instances {
            if inst.position == position {
                return true
            }
        }
        false
    }
    
    // TODO?: -> Result2<(), AlreadyOccupiedError, NotFoundError>
    fn shift(&mut self, from: Position, to: Position) -> Result<(), EntityError> {
        if self.occupied(to) {
            return Err(EntityError::AlreadyOccupied);
        }
        for inst in self.instances.iter_mut() {
            if inst.position == from {
                inst.position = to;
                return Ok(())
            }
        }
        Err(EntityError::NotFound)
    }
    
    fn removeAt(&mut self, position: Position) {
        self.instances.retain(|element| element.position != position);
    }
}


#[test]
fn instantiation() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    let c = Position::new(-1, 3);
    
    let mut ents = Entities::new();
    assert!(!ents.occupied(a));
    assert!(!ents.occupied(b));
    assert!(!ents.occupied(c));
    
    ents.add(Entity::new(a, ())).unwrap();
    assert!(ents.occupied(a));
    assert!(!ents.occupied(b));
    assert!(!ents.occupied(c));
    
    ents.add(Entity::new(b, ())).unwrap();
    assert!(ents.occupied(a));
    assert!(ents.occupied(b));
    assert!(!ents.occupied(c));
}

#[test]
fn movement() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    
    let mut ents = Entities::new();
    
    ents.add(Entity::new(a, ())).unwrap();
    assert!(ents.occupied(a));
    assert!(!ents.occupied(b));
    
    ents.shift(a, b).unwrap();
    assert!(!ents.occupied(a));
    assert!(ents.occupied(b));
}

#[test]
fn movement_nonexistent() {
    let mut ents: Entities<()> = Entities::new();
    
    match ents.shift(Position::new(0, 1), Position::new(2, 5)) {
        Err(EntityError::NotFound) => (),
        _ => panic!(),
    }
}

#[test]
fn already_occupied() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    let mut ents = Entities::new();
    
    ents.add(Entity::new(a, ())).unwrap();
    ents.add(Entity::new(b, ())).unwrap();
    
    match ents.add(Entity::new(a, ())) {
        Err(EntityError::AlreadyOccupied) => (),
        _ => panic!(),
    }
    assert!(ents.occupied(a));
    assert!(ents.occupied(b));
    
    match ents.shift(a, b) {
        Err(EntityError::AlreadyOccupied) => (),
        _ => panic!(),
    }
    assert!(ents.occupied(a));
    assert!(ents.occupied(b));
}

#[test]
fn position_removal() {
    let a = Position::new(0, 0);
    let b = Position::new(2, 0);
    let mut ents = Entities::new();
    
    ents.add(Entity::new(a, ())).unwrap();
    ents.add(Entity::new(b, ())).unwrap();
    
    assert!(ents.occupied(a));
    assert!(ents.occupied(b));
    
    ents.removeAt(a);
    assert!(!ents.occupied(a));
    assert!(ents.occupied(b));
}

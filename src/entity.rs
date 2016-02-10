use space::*;
use space::Direction::*;
use std::io::{Read, Write};
use std::collections::HashMap;


#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Entity(u32);


pub struct Entities {
    next_id: u32,
}
impl Entities {
    pub fn new() -> Entities {
        Entities {
            next_id: 0,
        }
    }
    
    pub fn make(&mut self) -> Entity {
        let result = Entity(self.next_id);
        self.next_id += 1;
        result
    }
}


pub struct Components<R> {
    pub contents: HashMap<Entity, R>
}
impl<R> Components<R> {
    pub fn new() -> Components<R> {
        Components {
            contents: HashMap::new()
        }
    }
    
    pub fn attach(&mut self, entity: Entity, component: R) {
        self.contents.insert(entity, component);
    }
    
    pub fn attached(&self, entity: Entity) -> bool {
        self.contents.contains_key(&entity)
    }
    
    pub fn detach(&mut self, entity: Entity) {
        self.contents.remove(&entity);
    }
    
    pub fn of_ref(&self, entity: Entity) -> Option<&R> {
        self.contents.get(&entity)
    }
    
    pub fn of_mut_ref(&mut self, entity: Entity) -> Option<&mut R> {
        self.contents.get_mut(&entity)
    }
}

impl<R: Clone> Components<R> {
    pub fn of(&self, entity: Entity) -> Option<R> {
        self.of_ref(entity).map(Clone::clone)
    }
}

#[test]
fn attachment() {
    let entity = Entity(0);
    let mut group = Components::new();
    assert!(!group.attached(entity));
    assert_eq!(group.of(entity), None);
    
    group.attach(entity, 99);
    assert!(group.attached(entity));
    assert_eq!(group.of(entity), Some(99));
    
    group.detach(entity);
    assert!(!group.attached(entity));
    assert_eq!(group.of(entity), None);
}

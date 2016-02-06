use space::*;
use space::Direction::*;
use std::io::{Read, Write};
use std::collections::HashMap;


#[derive(Copy, Clone, Hash, PartialEq, Eq)]
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


pub struct Components<R>(HashMap<Entity, R>);
impl<R> Components<R> {
    pub fn new() -> Components<R> {
        Components(HashMap::new())
    }
    
    pub fn attach(&mut self, entity: Entity, component: R) {
        self.map_mut().insert(entity, component);
    }
    
    pub fn attached(&self, entity: Entity) -> bool {
        self.map().contains_key(&entity)
    }
    
    pub fn detach(&mut self, entity: Entity) {
        self.map_mut().remove(&entity);
    }
    
    pub fn map(&self) -> &HashMap<Entity, R> {
        let &Components(ref result) = self;
        result
    }
    
    pub fn map_mut(&mut self) -> &mut HashMap<Entity, R> {
        let &mut Components(ref mut result) = self;
        result
    }
    
    pub fn of_ref(&self, entity: Entity) -> Option<&R> {
        self.map().get(&entity)
    }
    
    pub fn of_mut_ref(&mut self, entity: Entity) -> Option<&mut R> {
        self.map_mut().get_mut(&entity)
    }
}

impl<R: Clone> Components<R> {
    pub fn of(&self, entity: Entity) -> Option<R> {
        self.of_ref(entity).map(Clone::clone)
    }
}

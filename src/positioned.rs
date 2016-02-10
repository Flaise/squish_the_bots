use space::*;
use entity::*;


impl Components<Position> {
    pub fn at(&self, focus: Position) -> Option<Entity> {
        for (entity, position) in &self.contents {
            if *position == focus {
                return Some(*entity);
            }
        }
        None
    }
    
    pub fn occupied(&self, focus: Position) -> bool {
        self.at(focus) != None
    }
    
    pub fn set(&mut self, entity: Entity, destination: Position) {
        match self.of_mut_ref(entity) {
            None => (), // Shouldn't happen
            Some(position) => *position = destination,
        }
    }
}

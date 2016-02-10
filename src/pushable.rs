use space::*;
use area::*;
use entity::*;


pub enum PushResult {
    Success,
    TooHeavy,
    DestroysEnterer,
}
#[derive(Copy, Clone, PartialEq)]
pub enum Pushable {
    Squishable,
    Heavy,
    DestroysEnterer,
}
impl Area {
    fn push_impl(&mut self, target: Position, direction: Direction, chain: u8) -> PushResult {
        match self.positions.at(target) {
            None => return PushResult::Success,
            Some(entity) => {
                let destination = target + direction;
                
                match self.pushables.of(entity) {
                    None => PushResult::TooHeavy,
                    Some(Pushable::Squishable) => {
                        if chain > 1 {
                            PushResult::TooHeavy
                        }
                        else {
                            match self.push_impl(destination, direction, chain + 1) {
                                PushResult::Success => self.positions.set(entity, destination),
                                PushResult::TooHeavy | PushResult::DestroysEnterer => {
                                    self.remove(entity)
                                }
                            }
                            PushResult::Success
                        }
                    },
                    Some(Pushable::Heavy) => {
                        if chain > 0 {
                            PushResult::TooHeavy
                        }
                        else {
                            match self.push_impl(destination, direction, chain + 1) {
                                PushResult::Success => {
                                    self.positions.set(entity, destination);
                                    PushResult::Success
                                },
                                PushResult::TooHeavy => PushResult::TooHeavy,
                                PushResult::DestroysEnterer => {
                                    self.remove(entity);
                                    PushResult::Success
                                }
                            }
                        }
                    },
                    Some(Pushable::DestroysEnterer) => PushResult::DestroysEnterer,
                }
                
            }
        }
    }
    
    pub fn push(&mut self, target: Position, direction: Direction) -> PushResult {
        self.push_impl(target, direction, 0)
    }
}


impl Area {
    pub fn go(&mut self, entity: Entity, direction: Direction) -> Option<PushResult> {
        match self.positions.of(entity) {
            None => None,
            Some(position) => {
                let destination = position + direction;
                let push_result = self.push(destination, direction);
                match push_result {
                    PushResult::Success => self.positions.set(entity, destination),
                    PushResult::DestroysEnterer => self.remove(entity),
                    PushResult::TooHeavy => (),
                };
                Some(push_result)
            }
        }
    }
}

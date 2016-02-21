use space::*;
use area::*;
use entity::*;


pub enum DrillResult {
    Success,
    DestroysEnterer,
}


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
    
    fn push(&mut self, target: Position, direction: Direction) -> PushResult {
        self.push_impl(target, direction, 0)
    }
    
    pub fn go(&mut self, entity: Entity, direction: Direction) -> Option<PushResult> {
        match self.positions.of(entity) {
            None => {
                debug_unreachable!();
                None
            }
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
    
    fn do_drill(&mut self, target: Position) -> DrillResult {
        match self.positions.at(target) {
            None => return DrillResult::Success,
            Some(entity) => {
                match self.pushables.of(entity) {
                    None => DrillResult::Success,
                    Some(Pushable::Squishable) | Some(Pushable::Heavy) => {
                        self.remove(entity);
                        DrillResult::Success
                    }
                    Some(Pushable::DestroysEnterer) => DrillResult::DestroysEnterer
                }
            }
        }
    }
    
    pub fn drill(&mut self, entity: Entity, direction: Direction) -> Option<DrillResult> {
        match self.positions.of(entity) {
            None => {
                debug_unreachable!();
                None
            }
            Some(position) => {
                let destination = position + direction;
                let push_result = self.do_drill(destination);
                match push_result {
                    DrillResult::Success => self.positions.set(entity, destination),
                    DrillResult::DestroysEnterer => self.remove(entity),
                };
                Some(push_result)
            }
        }
    }
}

#[test]
fn test_drill() {
    for dir in Direction::all() {
        for component in vec![Pushable::Squishable, Pushable::Heavy] {
            let origin = Position::zero();
            let destination = origin + dir;
            
            let mut area = Area::new();
            
            let bot = area.entities.make();
            area.positions.attach(bot, origin);
            
            let target = area.entities.make();
            area.positions.attach(target, destination);
            area.pushables.attach(target, Pushable::Squishable);
            
            area.drill(bot, dir);
            assert_eq!(area.positions.of(bot), Some(destination));
            assert_eq!(area.positions.of(target), None);
        }
        
        {
            let origin = Position::zero();
            let destination = origin + dir;
            
            let mut area = Area::new();
            
            let bot = area.entities.make();
            area.positions.attach(bot, origin);
            
            let target = area.entities.make();
            area.positions.attach(target, destination);
            area.pushables.attach(target, Pushable::DestroysEnterer);
            
            area.drill(bot, dir);
            assert_eq!(area.positions.of(bot), None);
            assert_eq!(area.positions.of(target), Some(destination));
        }
    }
}

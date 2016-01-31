use space::*;
use space::Direction::*;


#[derive(PartialEq, Debug)]
enum EntityType {
    Bot,
    Block,
    Abyss,
}


enum PushResult {
    Success,
    TooHeavy,
    Abyss,
}


enum Entity {
    Bot {
        position: Position,
        ticks_until_action: u8,
    },
    Block {
        position: Position,
    },
    Abyss {
        position: Position,
    },
}


pub struct Area {
    contents: Vec<Entity>,
    bounds: Rectangle,
    ticks_elapsed: u16,
}
impl Area {
    fn new(bounds: Rectangle) -> Area {
        Area {
            contents: Vec::new(),
            bounds: bounds,
            ticks_elapsed: 0,
        }
    }
    
    fn make(&mut self, position: Position, entType: EntityType) {
        let entity = match entType {
            EntityType::Bot => Entity::Bot { position: position, ticks_until_action: 0 },
            EntityType::Block => Entity::Block { position: position },
            EntityType::Abyss => Entity::Abyss { position: position },
        };
        self.contents.push(entity);
    }
    
    fn remove(&mut self, position: Position) {
        self.contents.retain(|element| {
            position != match *element {
                Entity::Bot { position, .. } => position,
                Entity::Block { position } => position,
                Entity::Abyss { position } => position,
            }
        });
    }
    
    fn shift(&mut self, source: Position, destination: Position) {
        for element in self.contents.iter_mut() {
            match *element {
                Entity::Bot { position, ticks_until_action } if position == source => {
                    *element = Entity::Bot {
                        position: destination,
                        ticks_until_action: ticks_until_action,
                    };
                    return;
                },
                Entity::Block { position } if position == source => {
                    *element = Entity::Block { position: destination };
                    return;
                },
                _ => (),
            }
        }
    }
    
    fn push(&mut self, focus: Position, direction: Direction) -> PushResult {
        let destination = focus + direction;
        
        match self.type_at(focus) {
            None => PushResult::Success,
            Some(EntityType::Abyss) => PushResult::Abyss,
            Some(EntityType::Bot) => {
                match self.type_at(destination) {
                    None => {
                        self.shift(focus, destination);
                        PushResult::Success
                    },
                    Some(_) => {
                        self.remove(focus);
                        PushResult::Success
                    }
                }
            },
            Some(EntityType::Block) => {
                match self.type_at(destination) {
                    None => {
                        self.shift(focus, destination);
                        PushResult::Success
                    },
                    Some(EntityType::Block) => PushResult::TooHeavy,
                    Some(EntityType::Abyss) => {
                        self.remove(focus);
                        PushResult::Success
                    },
                    Some(EntityType::Bot) => {
                        let destination2 = destination + direction;
                        match self.type_at(destination2) {
                            None => {
                                self.shift(destination, destination2);
                                self.shift(focus, destination);
                                PushResult::Success
                            },
                            Some(_) => {
                                self.remove(destination);
                                self.shift(focus, destination);
                                PushResult::Success
                            },
                        }
                    },
                }
            },
        }
    }
    
    fn bot_go(&mut self, source: Position, direction: Direction) {
        let destination = source + direction;
        match self.push(destination, direction) {
            PushResult::Success => self.shift(source, destination),
            PushResult::Abyss => self.remove(source),
            PushResult::TooHeavy => (),
        }
    }
    
    fn bot_drill(&mut self, position: Position, direction: Direction) {
        
    }
    
    fn type_at(&self, dest: Position) -> Option<EntityType> {
        if !self.bounds.contains(dest) {
            return Some(EntityType::Abyss)
        }
        for element in &self.contents {
            match *element {
                Entity::Bot { position, .. } if dest == position => return Some(EntityType::Bot),
                Entity::Block { position } if dest == position => return Some(EntityType::Block),
                Entity::Abyss { position } if dest == position => return Some(EntityType::Abyss),
                _ => (),
            }
        }
        None
    }
    
    fn occupied(&self, position: Position) -> bool {
        match self.type_at(position) {
            Some(_) => true,
            None => false,
        }
    }
    
    fn tick(&mut self) {
        self.ticks_elapsed += 1;
    }
}


fn test_data() -> (Position, Position, Position, Area) {
    (
        Position::new(0, 0),
        Position::new(2, 0),
        Position::new(5, 4),
        Area::new(Rectangle::wh(10 * East + 10 * South)),
    )
}


#[test]
fn occupation() {
    let (a, b, c, mut area) = test_data();
    
    area.make(a, EntityType::Bot);
    assert!(area.occupied(a));
    assert_eq!(area.type_at(a), Some(EntityType::Bot));
    assert!(!area.occupied(b));
    assert!(!area.occupied(c));
    
    area.make(b, EntityType::Block);
    assert!(area.occupied(a));
    assert_eq!(area.type_at(a), Some(EntityType::Bot));
    assert!(area.occupied(b));
    assert_eq!(area.type_at(b), Some(EntityType::Block));
    assert!(!area.occupied(c));
}

#[test]
fn abyss_around_arena() {
    let (_, _, _, area) = test_data();
    
    assert_eq!(area.type_at(Position::new(0, 0)), None);
    assert_eq!(area.type_at(Position::new(-1, 0)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(0, -1)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(10, 0)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(0, 10)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(10, 10)), Some(EntityType::Abyss));
}

#[test]
fn travelment() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        area.make(c, EntityType::Bot);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
    }
}

#[test]
fn pushing() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Block));
    }
}

#[test]
fn pushing_too_heavy() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.make(c + dir + dir, EntityType::Block);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir), Some(EntityType::Block));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Block));
    }
}

#[test]
fn skydiving() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Abyss);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Abyss));
    }
    
    for dir in vec![North, East, South, West] {
        let mut area = Area::new(Rectangle::corners(Position::zero(), Position::zero()));
        assert_eq!(area.type_at(Position::zero()), None);
        
        area.make(Position::zero(), EntityType::Bot);
        area.bot_go(Position::zero(), dir);
        assert_eq!(area.type_at(Position::zero()), None);
        assert_eq!(area.type_at(Position::zero() + dir), Some(EntityType::Abyss));
    }
}

#[test]
fn shove_into_abyss() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Bot);
        area.make(c + dir + dir, EntityType::Abyss);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Abyss));
        
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.make(c + dir + dir, EntityType::Abyss);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Abyss));
        
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.make(c + dir * 2, EntityType::Bot);
        area.make(c + dir * 3, EntityType::Abyss);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir * 2), Some(EntityType::Block));
        assert_eq!(area.type_at(c + dir * 3), Some(EntityType::Abyss));
        
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Bot);
        area.make(c + dir * 2, EntityType::Bot);
        area.make(c + dir * 3, EntityType::Abyss);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir * 2), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir * 3), Some(EntityType::Abyss));
    }
}

#[test]
fn squish_directly() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Bot);
        area.make(c + dir + dir, EntityType::Block);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Block));
    }
}

#[test]
fn squish_indirectly() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.make(c + dir + dir, EntityType::Bot);
        area.make(c + dir + dir + dir, EntityType::Block);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Block));
        assert_eq!(area.type_at(c + dir + dir + dir), Some(EntityType::Block));
        
        let (_, _, c, mut area) = test_data();
        area.make(c, EntityType::Bot);
        area.make(c + dir, EntityType::Block);
        area.make(c + dir + dir, EntityType::Bot);
        area.make(c + dir + dir + dir, EntityType::Bot);
        area.bot_go(c, dir);
        assert_eq!(area.type_at(c), None);
        assert_eq!(area.type_at(c + dir), Some(EntityType::Bot));
        assert_eq!(area.type_at(c + dir + dir), Some(EntityType::Block));
        assert_eq!(area.type_at(c + dir + dir + dir), Some(EntityType::Bot));
    }
}

#[test]
fn elapsation() {
    let (_, _, c, mut area) = test_data();
    assert_eq!(area.ticks_elapsed, 0);
    
    area.tick();
    assert_eq!(area.ticks_elapsed, 1);
    
    area.tick();
    assert_eq!(area.ticks_elapsed, 2);
}

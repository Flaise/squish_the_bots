use space::*;
use space::Direction::*;


#[derive(PartialEq, Debug)]
enum EntityType {
    Bot,
    Block,
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
    ticks_elapsed: u16
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
    
    fn bot_go(&mut self, position: Position, direction: Direction) {
        
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
}


fn test_positions() -> (Position, Position, Position) {
    (Position::new(0, 0), Position::new(2, 0), Position::new(5, 3))
}


#[test]
fn occupation() {
    let (a, b, c) = test_positions();
    let mut area = Area::new(Rectangle::wh(10 * East + 10 * South));
    
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
    let area = Area::new(Rectangle::wh(10 * East + 10 * South));
    assert_eq!(area.type_at(Position::new(0, 0)), None);
    assert_eq!(area.type_at(Position::new(-1, 0)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(0, -1)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(10, 0)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(0, 10)), Some(EntityType::Abyss));
    assert_eq!(area.type_at(Position::new(10, 10)), Some(EntityType::Abyss));
}

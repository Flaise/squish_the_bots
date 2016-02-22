use std::io::{Read, Write};
use appearance::*;
use space::*;
use space::Direction::*;
use entity::*;
use pushable::*;
use action::*;
use notification::*;


pub fn make_bot(area: &mut Area, position: Position) -> Entity {
    let entity = area.entities.make();
    area.positions.attach(entity, position);
    area.appearances.attach(entity, Appearance::Bot);
    area.pushables.attach(entity, Pushable::Squishable);
    entity
}
pub fn make_block(area: &mut Area, position: Position) -> Entity {
    let entity = area.entities.make();
    area.positions.attach(entity, position);
    area.appearances.attach(entity, Appearance::Block);
    area.pushables.attach(entity, Pushable::Heavy);
    entity
}
pub fn make_abyss(area: &mut Area, position: Position) -> Entity {
    let entity = area.entities.make();
    area.positions.attach(entity, position);
    area.appearances.attach(entity, Appearance::Abyss);
    area.pushables.attach(entity, Pushable::DestroysEnterer);
    entity
}


pub struct Area {
    pub positions: Components<Position>,
    pub appearances: Components<Appearance>,
    pub pushables: Components<Pushable>,
    pub inputs: Components<Box<Read>>,
    pub outputs: Components<Box<Write>>,
    pub cooldowns: Components<u8>,
    pub participants_in_waiting: Vec<(Box<Read>, Box<Write>)>,
    pub entities: Entities,
}
impl Area {
    pub fn new() -> Area {
        Area {
            positions: Components::new(),
            appearances: Components::new(),
            pushables: Components::new(),
            inputs: Components::new(),
            outputs: Components::new(),
            cooldowns: Components::new(),
            participants_in_waiting: Vec::new(),
            entities: Entities::new(),
        }
    }
    
    pub fn remove(&mut self, entity: Entity) {
        self.notify(entity, Notification::YouDied);
        
        match (self.inputs.detach(entity), self.outputs.detach(entity)) {
            (Some(input), Some(output)) => {
                self.participants_in_waiting.push((input, output));
            },
            _ => (), // TODO: change tests to allow debug_unreachable!() here
        }
        
        self.disconnect(entity);
    }
    
    pub fn disconnect(&mut self, entity: Entity) {
        self.positions.detach(entity);
        self.appearances.detach(entity);
        self.pushables.detach(entity);
        self.inputs.detach(entity);
        self.outputs.detach(entity);
        self.cooldowns.detach(entity);
    }
}


fn test_data() -> (Position, Position, Position, Area) {
    (Position::new(0, 0), Position::new(2, 0), Position::new(5, 4), Area::new())
}


#[test]
fn occupation() {
    let (a, b, c, mut area) = test_data();
    
    make_bot(&mut area, a);
    assert!(area.positions.occupied(a));
    assert!(!area.positions.occupied(b));
    assert!(!area.positions.occupied(c));
    
    make_block(&mut area, b);
    assert!(area.positions.occupied(a));
    assert!(area.positions.occupied(b));
    assert!(!area.positions.occupied(c));
}

#[test]
fn appearance() {
    let (a, b, c, mut area) = test_data();
    
    make_bot(&mut area, a);
    assert_eq!(area.appearance_at(a), Appearance::Bot);
    assert_eq!(area.appearance_at(b), Appearance::Floor);
    assert_eq!(area.appearance_at(c), Appearance::Floor);
    
    make_block(&mut area, b);
    assert_eq!(area.appearance_at(a), Appearance::Bot);
    assert_eq!(area.appearance_at(b), Appearance::Block);
    assert_eq!(area.appearance_at(c), Appearance::Floor);
}

#[test]
fn travelment() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        let bot = make_bot(&mut area, c);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.positions.of(bot), Some(c + dir));
    }
}

#[test]
fn pushing() {
    for dir in vec![North, East, South, West] {
        {
            let (_, _, c, mut area) = test_data();
            let bot = make_bot(&mut area, c);
            let block = make_block(&mut area, c + dir);
            area.go(bot, dir);
            assert_eq!(area.appearance_at(c), Appearance::Floor);
            assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
            assert_eq!(area.appearance_at(c + dir * 2), Appearance::Block);
            assert_eq!(area.positions.of(bot), Some(c + dir));
            assert_eq!(area.positions.of(block), Some(c + dir * 2));
        }
        
        {
            let (_, _, c, mut area) = test_data();
            let bot1 = make_bot(&mut area, c);
            let block = make_block(&mut area, c + dir);
            let bot2 = make_bot(&mut area, c + dir * 2);
            area.go(bot1, dir);
            assert_eq!(area.appearance_at(c), Appearance::Floor);
            assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
            assert_eq!(area.appearance_at(c + dir * 2), Appearance::Block);
            assert_eq!(area.appearance_at(c + dir * 3), Appearance::Bot);
            assert_eq!(area.positions.of(bot1), Some(c + dir));
            assert_eq!(area.positions.of(block), Some(c + dir * 2));
            assert_eq!(area.positions.of(bot2), Some(c + dir * 3));
        }
    }
}

#[test]
fn pushing_too_heavy() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        
        let bot = make_bot(&mut area, c);
        let block1 = make_block(&mut area, c + dir);
        let block2 = make_block(&mut area, c + dir + dir);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir), Appearance::Block);
        assert_eq!(area.appearance_at(c + dir + dir), Appearance::Block);
        assert_eq!(area.positions.of(bot), Some(c));
        assert_eq!(area.positions.of(block1), Some(c + dir));
        assert_eq!(area.positions.of(block2), Some(c + dir * 2));
    }
}

#[test]
fn skydiving() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        let bot = make_bot(&mut area, c);
        make_abyss(&mut area, c + dir);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Abyss);
    }
}

#[test]
fn shove_into_abyss() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        let bot = make_bot(&mut area, c);
        make_bot(&mut area, c + dir);
        make_abyss(&mut area, c + dir + dir);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir + dir), Appearance::Abyss);
        
        let (_, _, c, mut area) = test_data();
        let bot = make_bot(&mut area, c);
        make_block(&mut area, c + dir);
        make_abyss(&mut area, c + dir + dir);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir + dir), Appearance::Abyss);
        
        let (_, _, c, mut area) = test_data();
        let bot = make_bot(&mut area, c);
        make_block(&mut area, c + dir);
        make_bot(&mut area, c + dir * 2);
        make_abyss(&mut area, c + dir * 3);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir * 2), Appearance::Block);
        assert_eq!(area.appearance_at(c + dir * 3), Appearance::Abyss);
        
        let (_, _, c, mut area) = test_data();
        let bot = make_bot(&mut area, c);
        make_bot(&mut area, c + dir);
        make_bot(&mut area, c + dir * 2);
        make_abyss(&mut area, c + dir * 3);
        area.go(bot, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir * 2), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir * 3), Appearance::Abyss);
    }
}

#[test]
fn squish_directly() {
    for dir in vec![North, East, South, West] {
        let (_, _, c, mut area) = test_data();
        let bot1 = make_bot(&mut area, c);
        let bot2 = make_bot(&mut area, c + dir);
        let block = make_block(&mut area, c + dir + dir);
        area.go(bot1, dir);
        assert_eq!(area.appearance_at(c), Appearance::Floor);
        assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
        assert_eq!(area.appearance_at(c + dir + dir), Appearance::Block);
        assert_eq!(area.positions.of(bot1), Some(c + dir));
        assert_eq!(area.positions.of(block), Some(c + dir * 2));
        assert_eq!(area.positions.of(bot2), None);
    }
}

#[test]
fn squish_indirectly() {
    for dir in vec![North, East, South, West] {
        {
            let (_, _, c, mut area) = test_data();
            let bot1 = make_bot(&mut area, c);
            let block1 = make_block(&mut area, c + dir);
            let bot2 = make_bot(&mut area, c + dir + dir);
            let block2 = make_block(&mut area, c + dir + dir + dir);
            area.go(bot1, dir);
            assert_eq!(area.appearance_at(c), Appearance::Floor);
            assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
            assert_eq!(area.appearance_at(c + dir + dir), Appearance::Block);
            assert_eq!(area.appearance_at(c + dir + dir + dir), Appearance::Block);
            assert_eq!(area.positions.of(bot1), Some(c + dir));
            assert_eq!(area.positions.of(block1), Some(c + dir * 2));
            assert_eq!(area.positions.of(bot2), None);
            assert_eq!(area.positions.of(block2), Some(c + dir * 3));
        }
        
        {
            let (_, _, c, mut area) = test_data();
            let bot1 = make_bot(&mut area, c);
            let block = make_block(&mut area, c + dir);
            let bot2 = make_bot(&mut area, c + dir + dir);
            let bot3 = make_bot(&mut area, c + dir + dir + dir);
            area.go(bot1, dir);
            assert_eq!(area.appearance_at(c), Appearance::Floor);
            assert_eq!(area.appearance_at(c + dir), Appearance::Bot);
            assert_eq!(area.appearance_at(c + dir + dir), Appearance::Block);
            assert_eq!(area.appearance_at(c + dir + dir + dir), Appearance::Bot);
            assert_eq!(area.positions.of(bot1), Some(c + dir));
            assert_eq!(area.positions.of(block), Some(c + dir * 2));
            assert_eq!(area.positions.of(bot2), None);
            assert_eq!(area.positions.of(bot3), Some(c + dir * 3));
        }
    }
}

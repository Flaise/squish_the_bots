use rand::{thread_rng, Rng};
use std::io::{Read, Write, Cursor};
use std::iter::Filter;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::time::Duration;
use area::*;
use space::*;
use space::Direction::*;
use action::*;
use entity::*;
use notification::*;
use appearance::*;


pub fn execute_round(participants: &mut Vec<(Box<Read>, Box<Write>)>, delay: Duration) {
    let mut area = generate_area(participants.drain(..).collect::<Vec<_>>());
    
    let winners = area.act_all(delay);
    // TODO: maybe send win/draw condition notifications
    
    participants.extend(area.extract_io_pairs());
}


impl Area {
    fn extract_io_pairs(&mut self) -> Vec<(Box<Read>, Box<Write>)> {
        let mut readers: HashMap<Entity, Box<Read>> = HashMap::new();
        for (entity, reader) in self.inputs.contents.drain() {
            match readers.entry(entity) {
                Occupied(_) => debug_unreachable!(),
                Vacant(entry) => { entry.insert(reader); },
            };
        }
        
        let mut pairs: HashMap<Entity, (Box<Read>, Box<Write>)> = HashMap::new();
        for (entity, writer) in self.outputs.contents.drain() {
            match readers.remove(&entity) {
                None => debug_unreachable!(),
                Some(reader) => { pairs.insert(entity, (reader, writer)); },
            };
        }
        
        let mut result = vec![];
        for (_, pair) in pairs {
            result.push(pair);
        }
        for pair in self.participants_in_waiting.drain(..) {
            result.push(pair);
        }
        result
    }
}


fn generate_area(participants: Vec<(Box<Read>, Box<Write>)>) -> Area {
    let length = 10 + participants.len() as i32;
    let bounds = Rectangle::wh(East * length + South * length);
    let mut area = Area::new();
    
    let mut rng = thread_rng();
    let limit = rng.gen_range(bounds.area() / 4, bounds.area() * 7 / 8) as usize;
    
    let mut positions = random_unoccupied_position_list(&area, bounds, limit);
    
    for (reader, writer) in participants {
        match positions.pop() {
            None => debug_unreachable!(break),
            Some(position) => {
                let bot = make_bot(&mut area, position);
                
                area.inputs.attach(bot, reader);
                area.outputs.attach(bot, writer);
            }
        }
    }
    
    for position in positions {
        match rng.gen_range(0, 1) {
            0 => { make_block(&mut area, position); },
            1 => { make_abyss(&mut area, position); },
            _ => debug_unreachable!(),
        };
    }
    
    let outer_bounds = Rectangle::corners(
        Position::zero() + West + North,
        Position::zero() + East * (length + 1) + South * (length + 1)
    );
    for position in outer_bounds {
        if bounds.contains(position) {
            continue;
        }
        make_abyss(&mut area, position);
    }
    
    area
}

fn unoccupied_positions(area: &Area, bounds: Rectangle) -> Vec<Position> {
    bounds.into_iter().filter(|position| !area.positions.occupied(*position)).collect()
}

fn random_unoccupied_position(area: &Area, bounds: Rectangle) -> Option<Position> {
    thread_rng().choose(&unoccupied_positions(area, bounds)).map(Clone::clone)
}

fn random_unoccupied_position_list(area: &Area, bounds: Rectangle, limit: usize) -> Vec<Position> {
    // TODO: this only works on new, empty areas
    let mut rng = thread_rng();
    let mut positions = bounds.into_iter().collect::<Vec<_>>();
    rng.shuffle(&mut positions);
    positions.into_iter().take(limit).collect::<Vec<_>>()
}


#[test]
fn emptiness() {
    let area = Area::new();
    let rect = Rectangle::corners(Position::zero(), Position::zero());
    assert_eq!(unoccupied_positions(&area, rect), vec![Position::zero()]);
    for _ in 0..1000 {
        assert_eq!(random_unoccupied_position(&area, rect), Some(Position::zero()));
    }
    
    let area = Area::new();
    let rect = Rectangle::corners(Position::zero(), Position::zero() + South);
    assert_eq!(unoccupied_positions(&area, rect),
               vec![Position::zero(), Position::zero() + South]);
    for _ in 0..1000 {
        let position = random_unoccupied_position(&area, rect);
        assert!(position == Some(Position::zero()) || position == Some(Position::zero() + South));
    }
}

#[test]
fn generation() {
    for i in 0..100 {
        let num_part = i % 20 + 1;
        
        let mut participants = Vec::<(Box<Read>, Box<Write>)>::new();
        for _ in 0..num_part {
            participants.push((Box::new(Cursor::new(vec![])), Box::new(vec![])));
        }
        
        let length = 10 + participants.len() as i32;
        let bounds = Rectangle::wh(East * length + South * length);
        let lower_limit = bounds.area() / 4;
        let upper_limit = bounds.area() * 7 / 8;
        
        let area = generate_area(participants);
        
        assert_eq!(area.inputs.contents.len(), num_part);
        assert_eq!(area.outputs.contents.len(), num_part);
        
        assert!(area.positions.contents.len() >= lower_limit as usize);
        
        // TODO: non-positional boundaries
        // assert!(area.positions.contents.len() <= upper_limit as usize);
    }
}

#[test]
fn outer_boundaries() {
    for _ in 0..100 {
        let area = generate_area(vec![]);
        
        let length = 10;
        let bounds = Rectangle::wh(East * length + South * length);
        
        let outer_bounds = Rectangle::corners(
            Position::zero() + West + North,
            Position::zero() + East * (length + 1) + South * (length + 1)
        );
        
        for position in outer_bounds {
            if bounds.contains(position) {
                continue;
            }
            assert_eq!(area.appearance_at(position), Appearance::Abyss);
        }
    }
}

#[test]
fn single_round_disconnection() {
    let mut participants: Vec<(Box<Read>, Box<Write>)> = vec![
        (Box::new(Cursor::new([2, 1])), Box::new(vec![])),
        (Box::new(Cursor::new([])), Box::new(vec![])), // EoF causes disconnection
    ];
    
    execute_round(&mut participants, Duration::from_millis(0));
    
    assert_eq!(participants.len(), 1);
}

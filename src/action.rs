use std::io::Read;
use std::mem::transmute;
use std::thread;
use std::time::Duration;
use space::*;
use space::Direction::*;
use area::*;
use entity::*;
use pushable::*;
use notification::*;
use cooldown::*;
use cooldown::CooldownState::*;


#[derive(PartialEq, Debug)]
enum Command {
    LookAt(Offset),
    Move(Direction),
    Drill(Direction),
    Malformed,
    End,
}


const CODE_LOOK_AT: u8 = 1;
const CODE_MOVE: u8 = 2;
const CODE_DRILL: u8 = 3;

const CODE_NORTH: u8 = 0;
const CODE_EAST: u8 = 1;
const CODE_SOUTH: u8 = 2;
const CODE_WEST: u8 = 3;


fn code_to_direction(code: u8) -> Option<Direction> {
    match code {
        CODE_NORTH => Some(North),
        CODE_EAST => Some(East),
        CODE_SOUTH => Some(South),
        CODE_WEST => Some(West),
        _ => None,
    }
}


fn i8_into_u8(x: i8) -> u8 {
    unsafe { transmute(x) }
}
fn i8_from_u8(x: u8) -> i8 {
    unsafe { transmute(x) }
}


fn parse_next(bytes: &mut Read) -> Command {
    let mut buf = [0];
    
    // TODO: read_exact
    if bytes.read(&mut buf).ok() != Some(1) {
        return Command::End;
    }
    
    match buf[0] {
        CODE_LOOK_AT => {
            if bytes.read(&mut buf).ok() != Some(1) {
                return Command::End;
            }
            
            let dx = i8_from_u8(buf[0]) as i32;
            
            if bytes.read(&mut buf).ok() != Some(1) {
                return Command::End;
            }
            
            let dy = i8_from_u8(buf[0]) as i32;
            
            ///////////// Positive Y is north, positive X is east
            
            return Command::LookAt(East * dx + North * dy);
        },
        CODE_MOVE => {
            if bytes.read(&mut buf).ok() != Some(1) {
                return Command::End;
            }
            
            return match code_to_direction(buf[0]) {
                Some(direction) => Command::Move(direction),
                None => Command::Malformed,
            };
        },
        CODE_DRILL => {
            if bytes.read(&mut buf).ok() != Some(1) {
                return Command::End;
            }
            
            return match code_to_direction(buf[0]) {
                Some(direction) => Command::Drill(direction),
                None => Command::Malformed,
            };
        },
        _ => return Command::Malformed,
    }
}


impl Area {
    pub fn notify(&mut self, bot: Entity, notification: Notification) {
        let error = {
            let mut output = match self.outputs.of_mut_ref(bot) {
                None => return,
                Some(output) => output,
            };
            match notify(&mut output, notification) {
                Ok(_) => return,
                Err(error) => error,
            }
        };
        println!("{:?} disconnected during notify: {:?}", bot, error);
        self.disconnect(bot);
    }
    
    fn act(&mut self, bot: Entity) {
        if self.tick(bot) == Waiting {
            return;
        }
        
        self.notify(bot, Notification::YourTurn);
        
        let command = match self.inputs.of_mut_ref(bot) {
            None => return,
            Some(mut input) => parse_next(&mut input),
        };
        
        match command {
            Command::LookAt(offset) => {
                let here = match self.positions.of(bot) {
                    None => {
                        debug_unreachable!();
                        return;
                    }
                    Some(here) => here,
                };
                let notification = Notification::YouSee(self.appearance_at(here + offset));
                self.notify(bot, notification);
            }
            Command::Move(direction) => {
                let push_result = self.go(bot, direction);
                match push_result {
                    None => (),
                    Some(PushResult::Success) => self.notify(bot, Notification::Success),
                    Some(PushResult::TooHeavy) => self.notify(bot, Notification::TooHeavy),
                    Some(PushResult::DestroysEnterer) => (), // notified in remove() function
                };
            }
            Command::Drill(direction) => {
                let drill_result = self.drill(bot, direction);
                match drill_result {
                    None => (),
                    Some(DrillResult::Success) => self.notify(bot, Notification::Success),
                    Some(DrillResult::DestroysEnterer) => (), // notified in remove() function
                };
            }
            Command::Malformed | Command::End => self.disconnect(bot),
        }
    }
    
    pub fn all_actors(&self) -> Vec<Entity> {
        let mut result = Vec::with_capacity(self.inputs.contents.len());
        for (entity, _) in &self.inputs.contents {
            result.push(*entity);
        }
        result.sort();
        result
    }
    
    fn act_vec(&mut self, entities: Vec<Entity>) {
        for entity in entities {
            self.act(entity);
        }
    }
    
    // Returns the winners of the round
    pub fn act_all(&mut self, delay: Duration) -> Vec<Entity> {
        loop {
            let entities = self.all_actors();
            if entities.len() <= 1 {
                return entities;
            }
            self.act_vec(entities);
            
            thread::sleep(delay);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Command, parse_next, i8_into_u8,
                CODE_NORTH, CODE_EAST, CODE_WEST};
    use space::*;
    use space::Direction::*;
    use std::io::Cursor;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::time::Duration;
    use area::*;
    use notification::*;
    
    use super::super::tests::{SharedWrite};
    
    #[test]
    fn parsing() {
        let mut commands = Cursor::new([]);
        assert_eq!(parse_next(&mut commands), Command::End);
        
        let mut commands = Cursor::new([
            1, i8_into_u8(0i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::default()));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(1i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::default() + East));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(-1i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::default() + West + North));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(-2i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::default() + West * 2 + North));
        
        let mut commands = Cursor::new([2, 0]);
        assert_eq!(parse_next(&mut commands), Command::Move(North));
        
        let mut commands = Cursor::new([2, 1]);
        assert_eq!(parse_next(&mut commands), Command::Move(East));
        
        let mut commands = Cursor::new([2, 2]);
        assert_eq!(parse_next(&mut commands), Command::Move(South));
        
        let mut commands = Cursor::new([2, 3]);
        assert_eq!(parse_next(&mut commands), Command::Move(West));
        
        let mut commands = Cursor::new([2, 4]);
        assert_eq!(parse_next(&mut commands), Command::Malformed);
        
        let mut commands = Cursor::new([3, 0]);
        assert_eq!(parse_next(&mut commands), Command::Drill(North));
        
        let mut commands = Cursor::new([3, 1]);
        assert_eq!(parse_next(&mut commands), Command::Drill(East));
        
        let mut commands = Cursor::new([3, 2]);
        assert_eq!(parse_next(&mut commands), Command::Drill(South));
        
        let mut commands = Cursor::new([3, 3]);
        assert_eq!(parse_next(&mut commands), Command::Drill(West));
        
        let mut commands = Cursor::new([3, 4]);
        assert_eq!(parse_next(&mut commands), Command::Malformed);
    }
    
    #[test]
    fn malformed_commands() {
        let streams = vec![
            vec![],
            vec![88],
            vec![4],
            vec![3],
            vec![2],
            vec![1],
            vec![1, 0],
            vec![0],
        ];
        for data in streams {
            let mut area = Area::new();
            let bot = make_bot(&mut area, Position::default());
            area.inputs.attach(bot, Box::new(Cursor::new(data)));
            area.outputs.attach(bot, Box::new(vec![]));
            area.act(bot);
            
            assert_eq!(area.positions.of(bot), None);
            assert_eq!(area.participants_in_waiting.len(), 0);
        }
    }
    
    #[test]
    fn win() {
        let mut area = Area::new();
        let bot_a = make_bot(&mut area, Position::default());
        let bot_b = make_bot(&mut area, Position::default() + East);
        let block = make_block(&mut area, Position::default() + East * 2);
        
        area.inputs.attach(bot_a, Box::new(Cursor::new([2, 1])));
        area.inputs.attach(bot_b, Box::new(Cursor::new([])));
        
        area.outputs.attach(bot_a, Box::new(vec![]));
        area.outputs.attach(bot_b, Box::new(vec![]));
        
        let entities = area.all_actors();
        assert_eq!(entities, &[bot_a, bot_b]);
        
        let winners = area.act_all(Duration::from_millis(0));
        assert_eq!(winners, &[bot_a]);
        
        let entities = area.all_actors();
        assert_eq!(entities, &[bot_a]);
        
        assert_eq!(area.positions.of(bot_a), Some(Position::default() + East));
        assert_eq!(area.positions.of(bot_b), None);
        assert_eq!(area.positions.of(block), Some(Position::default() + East * 2));
        
        assert_eq!(area.participants_in_waiting.len(), 1);
    }
    
    #[test]
    fn command_feedback() {
        let streams = vec![
            (vec![2, CODE_NORTH], vec![1, 2]), // died
            (vec![2, CODE_EAST], vec![1, 3]), // success
            (vec![2, CODE_WEST], vec![1, 4]), // too heavy
            (vec![1, i8_into_u8(0), i8_into_u8(-1)], vec![1, 6, 0]), // floor
            (vec![1, i8_into_u8(0), i8_into_u8(0)], vec![1, 6, 1]), // bot
            (vec![1, i8_into_u8(-1), i8_into_u8(0)], vec![1, 6, 2]), // wall
            (vec![1, i8_into_u8(0), i8_into_u8(1)], vec![1, 6, 3]), // abyss
        ];
        
        for (instream, target) in streams {
            let output = Rc::new(RefCell::new(Vec::<u8>::new()));
            {
                let mut area = Area::new();
                let bot = make_bot(&mut area, Position::default());
                area.inputs.attach(bot, Box::new(Cursor::new(instream)));
                make_abyss(&mut area, Position::default() + North);
                make_block(&mut area, Position::default() + West);
                make_block(&mut area, Position::default() + West * 2);
                
                let shared = SharedWrite::new(output.clone());
                area.outputs.attach(bot, Box::new(shared));
                
                area.act(bot);
            }
            
            assert_eq!(Rc::try_unwrap(output).unwrap().into_inner(), target);
        }
    }
    
    #[test]
    fn disconnect_during_notify() {
        let output = Rc::new(RefCell::new(Vec::<u8>::new()));
        let mut shared = SharedWrite::new(output.clone());
        drop(output);
        shared.close();
        
        let mut area = Area::new();
        let bot = make_bot(&mut area, Position::default());
        
        area.outputs.attach(bot, Box::new(shared));
        
        area.notify(bot, Notification::YourTurn);
        
        assert_eq!(area.outputs.contents.len(), 0);
    }
    
    #[test]
    fn cooldown_after_movement() {
        let mut area = Area::new();
        
        let mut position = Position::default();
        let bot = make_bot(&mut area, position);
        area.inputs.attach(bot, Box::new(Cursor::new(vec![
            2, 1,
            2, 1,
            2, 1,
        ])));
        
        for _ in 0..3 {
            position = position + East;
            for _ in 0..3 {
                area.act(bot);
                assert_eq!(area.positions.of(bot), Some(position));
            }
        }
    }
    
    #[test]
    fn cooldown_after_drilling() {
        let mut area = Area::new();
        
        let mut position = Position::default();
        let bot = make_bot(&mut area, position);
        area.inputs.attach(bot, Box::new(Cursor::new(vec![
            3, 1,
            3, 1,
            3, 1,
        ])));
        
        for _ in 0..3 {
            position = position + East;
            for _ in 0..5 {
                area.act(bot);
                assert_eq!(area.positions.of(bot), Some(position));
            }
        }
    }
}

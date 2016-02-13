use std::io::{Read, Write};
use std::mem::transmute;
use space::*;
use space::Direction::*;
use area::*;
use appearance::*;
use entity::*;
use pushable::*;


#[derive(PartialEq, Debug)]
enum Command {
    LookAt(Offset),
    Move(Direction),
    Drill(Direction),
    Malformed,
    End,
}


#[derive(PartialEq, Debug)]
pub enum Notification {
    YourTurn,
    YouSee(Appearance),
    YouDied,
    Success,
    TooHeavy,
    NewRound,
}


const CodeLookAt: u8 = 1;
const CodeMove: u8 = 2;
const CodeDrill: u8 = 3;

const CodeNorth: u8 = 0;
const CodeEast: u8 = 1;
const CodeSouth: u8 = 2;
const CodeWest: u8 = 3;

const CodeFloor: u8 = 0;
const CodeBot: u8 = 1;
const CodeBlock: u8 = 2;
const CodeAbyss: u8 = 3;


fn code_to_direction(code: u8) -> Option<Direction> {
    match code {
        CodeNorth => Some(North),
        CodeEast => Some(East),
        CodeSouth => Some(South),
        CodeWest => Some(West),
        _ => None,
    }
}

fn appearance_to_code(appearance: Appearance) -> u8 {
    match appearance {
        Appearance::Floor => CodeFloor,
        Appearance::Bot => CodeBot,
        Appearance::Block => CodeBlock,
        Appearance::Abyss => CodeAbyss,
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
        CodeLookAt => {
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
        CodeMove => {
            if bytes.read(&mut buf).ok() != Some(1) {
                return Command::End;
            }
            
            return match code_to_direction(buf[0]) {
                Some(direction) => Command::Move(direction),
                None => Command::Malformed,
            };
        },
        CodeDrill => {
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

fn serialize_notification(notification: Notification) -> Vec<u8> {
    match notification {
        Notification::YourTurn => vec![1],
        Notification::YouDied => vec![2],
        Notification::Success => vec![3],
        Notification::TooHeavy => vec![4],
        Notification::NewRound => vec![5],
        Notification::YouSee(appearance) => vec![6, appearance_to_code(appearance)],
    }
}


impl Area {
    pub fn notify(&mut self, bot: Entity, notification: Notification) {
        let error = {
            let output = match self.outputs.of_mut_ref(bot) {
                None => return,
                Some(output) => output,
            };
            match output.write_all(serialize_notification(notification).as_ref()) {
                Ok(_) => return,
                Err(error) => error,
            }
        };
        println!("WARNING: {:?}", error);
        self.remove(bot);
    }
    
    fn act(&mut self, bot: Entity) {
        self.notify(bot, Notification::YourTurn);
        
        let command = match self.inputs.of_mut_ref(bot) {
            None => return,
            Some(mut input) => parse_next(&mut input),
        };
        
        match command {
            Command::LookAt(offset) => {
                let here = match self.positions.of(bot) {
                    None => debug_unreachable!(return),
                    Some(here) => here,
                };
                let notification = Notification::YouSee(self.appearance_at(here + offset));
                self.notify(bot, notification);
            },
            Command::Move(direction) => {
                let push_result = self.go(bot, direction);
                match push_result {
                    None => (),
                    Some(PushResult::Success) => self.notify(bot, Notification::Success),
                    Some(PushResult::TooHeavy) => self.notify(bot, Notification::TooHeavy),
                    Some(PushResult::DestroysEnterer) => (), // notified in remove() function
                };
            },
            Command::Drill(direction) => (),//self.bot_drill(bot, direction),
            Command::Malformed | Command::End => self.disconnect(bot),
        }
    }
    
    pub fn all_actors(&self) -> Vec<Entity> {
        let mut result = Vec::with_capacity(self.inputs.contents.len());
        for (entity, input) in &self.inputs.contents {
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
    pub fn act_all(&mut self) -> Vec<Entity> {
        loop {
            let entities = self.all_actors();
            if entities.len() <= 1 {
                return entities;
            }
            self.act_vec(entities);
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    use super::{Command, parse_next, i8_into_u8, serialize_notification, Notification,
                CodeNorth, CodeEast, CodeSouth, CodeWest};
    use space::*;
    use space::Direction::*;
    use entity::*;
    use std::io;
    use std::io::{Write, Cursor};
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;
    use area::*;
    use appearance::*;
    use std::borrow::Borrow;
    
    #[test]
    fn parsing() {
        let mut commands = Cursor::new([]);
        assert_eq!(parse_next(&mut commands), Command::End);
        
        let mut commands = Cursor::new([
            1, i8_into_u8(0i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero()));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(1i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + East));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(-1i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + West + North));
        
        let mut commands = Cursor::new([
            1, i8_into_u8(-2i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + West * 2 + North));
        
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
    fn serializing() {
        assert_eq!(serialize_notification(Notification::YourTurn), vec![1]);
        assert_eq!(serialize_notification(Notification::YouDied), vec![2]);
        assert_eq!(serialize_notification(Notification::Success), vec![3]);
        assert_eq!(serialize_notification(Notification::TooHeavy), vec![4]);
        assert_eq!(serialize_notification(Notification::NewRound), vec![5]);
        assert_eq!(serialize_notification(Notification::YouSee(Appearance::Floor)), vec![6, 0]);
        assert_eq!(serialize_notification(Notification::YouSee(Appearance::Bot)),
                   vec![6, 1]);
        assert_eq!(serialize_notification(Notification::YouSee(Appearance::Block)),
                   vec![6, 2]);
        assert_eq!(serialize_notification(Notification::YouSee(Appearance::Abyss)),
                   vec![6, 3]);
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
            let bot = make_bot(&mut area, Position::zero());
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
        let botA = make_bot(&mut area, Position::zero());
        let botB = make_bot(&mut area, Position::zero() + East);
        let block = make_block(&mut area, Position::zero() + East * 2);
        
        area.inputs.attach(botA, Box::new(Cursor::new([2, 1])));
        area.inputs.attach(botB, Box::new(Cursor::new([])));
        
        area.outputs.attach(botA, Box::new(vec![]));
        area.outputs.attach(botB, Box::new(vec![]));
        
        let entities = area.all_actors();
        assert_eq!(entities, &[botA, botB]);
        
        let winners = area.act_all();
        assert_eq!(winners, &[botA]);
        
        let entities = area.all_actors();
        assert_eq!(entities, &[botA]);
        
        assert_eq!(area.positions.of(botA), Some(Position::zero() + East));
        assert_eq!(area.positions.of(botB), None);
        assert_eq!(area.positions.of(block), Some(Position::zero() + East * 2));
        
        assert_eq!(area.participants_in_waiting.len(), 1);
    }
    
    #[test]
    fn command_feedback() {
        let streams = vec![
            (vec![2, CodeNorth], vec![1, 2]), // died
            (vec![2, CodeEast], vec![1, 3]), // success
            (vec![2, CodeWest], vec![1, 4]), // too heavy
            (vec![1, i8_into_u8(0), i8_into_u8(-1)], vec![1, 6, 0]), // floor
            (vec![1, i8_into_u8(0), i8_into_u8(0)], vec![1, 6, 1]), // bot
            (vec![1, i8_into_u8(-1), i8_into_u8(0)], vec![1, 6, 2]), // wall
            (vec![1, i8_into_u8(0), i8_into_u8(1)], vec![1, 6, 3]), // abyss
        ];
        
        for (instream, outstream) in streams {
            let output = Rc::new(RefCell::new(Vec::<u8>::new()));
            {
                let mut area = Area::new();
                let bot = make_bot(&mut area, Position::zero());
                area.inputs.attach(bot, Box::new(Cursor::new(instream)));
                make_abyss(&mut area, Position::zero() + North);
                make_block(&mut area, Position::zero() + West);
                make_block(&mut area, Position::zero() + West * 2);
                
                let shared = SharedWrite { writer: output.clone() };
                area.outputs.attach(bot, Box::new(shared));
                
                area.act(bot);
            }
            
            assert_eq!(Rc::try_unwrap(output).unwrap().into_inner(), outstream);
        }
    }
    
    struct SharedWrite {
        writer: Rc<RefCell<Write>>,
    }
    impl Write for SharedWrite {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.writer.borrow_mut().write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.writer.borrow_mut().flush()
        }
    }
}

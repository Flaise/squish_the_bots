use std::io::Read;
use std::mem::transmute;
use space::*;
use space::Direction::*;
use area::*;
use appearance::*;
use entity::*;


#[derive(PartialEq, Debug)]
enum Command {
    LookAt(Offset),
    Move(Direction),
    Drill(Direction),
    Malformed,
    End,
}


#[derive(PartialEq, Debug)]
enum Notification {
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
    fn act(&mut self, bot: Entity) {
        let command = match self.inputs.of_mut_ref(bot) {
            None => return, // shouldn't happen
            Some(mut input) => parse_next(&mut input),
        };
        
        match command {
            Command::LookAt(offset) => match self.positions.of(bot) {
                None => (), // shouldn't happen
                Some(here) => (),
                
                // notify(Notification::YouSee(
                //     self.appearance_at(here + offset)
                // ))
            },
            Command::Move(direction) => self.go(bot, direction),
            Command::Drill(direction) => (),//self.bot_drill(bot, direction),
            Command::Malformed | Command::End => self.remove(bot),
        }
    }
    
    fn all_actors(&self) -> Vec<Entity> {
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
    fn act_all(&mut self) -> Vec<Entity> {
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
    use super::{Command, parse_next, i8_into_u8, serialize_notification, Notification};
    use space::*;
    use space::Direction::*;
    use entity::*;
    use std::io::Write;
    use std::rc::Rc;
    use area::*;
    use appearance::*;
    
    extern crate memstream;
    use self::memstream::*;
    
    
    fn stream_from_slice(slice: &[u8]) -> MemStream {
        let mut result = MemStream::new();
        result.write(slice).unwrap();
        result
    }
    
    #[test]
    fn parsing() {
        let mut commands = stream_from_slice(&[]);
        assert_eq!(parse_next(&mut commands), Command::End);
        
        let mut commands = stream_from_slice(&[
            1, i8_into_u8(0i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero()));
        
        let mut commands = stream_from_slice(&[
            1, i8_into_u8(1i8), i8_into_u8(0i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + East));
        
        let mut commands = stream_from_slice(&[
            1, i8_into_u8(-1i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + West + North));
        
        let mut commands = stream_from_slice(&[
            1, i8_into_u8(-2i8), i8_into_u8(1i8)
        ]);
        assert_eq!(parse_next(&mut commands), Command::LookAt(Offset::zero() + West * 2 + North));
        
        let mut commands = stream_from_slice(&[2, 0]);
        assert_eq!(parse_next(&mut commands), Command::Move(North));
        
        let mut commands = stream_from_slice(&[2, 1]);
        assert_eq!(parse_next(&mut commands), Command::Move(East));
        
        let mut commands = stream_from_slice(&[2, 2]);
        assert_eq!(parse_next(&mut commands), Command::Move(South));
        
        let mut commands = stream_from_slice(&[2, 3]);
        assert_eq!(parse_next(&mut commands), Command::Move(West));
        
        let mut commands = stream_from_slice(&[2, 4]);
        assert_eq!(parse_next(&mut commands), Command::Malformed);
        
        let mut commands = stream_from_slice(&[3, 0]);
        assert_eq!(parse_next(&mut commands), Command::Drill(North));
        
        let mut commands = stream_from_slice(&[3, 1]);
        assert_eq!(parse_next(&mut commands), Command::Drill(East));
        
        let mut commands = stream_from_slice(&[3, 2]);
        assert_eq!(parse_next(&mut commands), Command::Drill(South));
        
        let mut commands = stream_from_slice(&[3, 3]);
        assert_eq!(parse_next(&mut commands), Command::Drill(West));
        
        let mut commands = stream_from_slice(&[3, 4]);
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
    fn disappear() {
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
        for command_stream in streams {
            let mut area = Area::new();
            let bot = make_bot(&mut area, Position::zero());
            area.inputs.attach(bot, Box::new(stream_from_slice(&command_stream)));
            area.act(bot);
            
            assert_eq!(area.positions.of(bot), None);
        }
    }
    
    #[test]
    fn win() {
        let mut area = Area::new();
        let botA = make_bot(&mut area, Position::zero());
        let botB = make_bot(&mut area, Position::zero() + East);
        let block = make_block(&mut area, Position::zero() + East * 2);
        
        area.inputs.attach(botA, Box::new(stream_from_slice(&[2, 1])));
        area.inputs.attach(botB, Box::new(stream_from_slice(&[])));
        
        let entities = area.all_actors();
        assert_eq!(entities, &[botA, botB]);
        area.act_vec(entities);
        
        let entities = area.all_actors();
        assert_eq!(entities, &[botA]);
        
        assert_eq!(area.positions.of(botA), Some(Position::zero() + East));
        assert_eq!(area.positions.of(botB), None);
        assert_eq!(area.positions.of(block), Some(Position::zero() + East * 2));
        
        
        // let winners = area.act_all();
        
        // assert_eq!(winners, &[botA]);
    }
    
    // #[test]
    // fn notification_death() {
    //     let a = Position::zero();
    //
    //     let mut commands = Rc::new(MemStream::new());
    //     let mut notifications = Rc::new(MemStream::new());
    //
    //     let mut area = Area::new(Rectangle::wh(North * 10 + East * 10));
    //     area.make(a, EntityType::Bot);
    //     area.bot_observe(a, commands.clone(), notifications.clone());
    //
    //     // assert_eq!(notifications.unwrap(), vec![]);
    //     area.bot_go(a, West);
    //     // assert_eq!(notifications.unwrap(), vec![2]);
    // }
}

use space::*;
use space::Direction::*;
use entity::*;
use std::io::Read;
use std::mem::transmute;


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
    YouSee(Option<EntityType>),
    YouDied,
    ActionSuccessful,
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

fn entity_type_to_code(entity_type: EntityType) -> u8 {
    match entity_type {
        EntityType::Bot => CodeBot,
        EntityType::Block => CodeBlock,
        EntityType::Abyss => CodeAbyss,
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
    
    match bytes.read(&mut buf) {
        Ok(1) => (),
        _ => return Command::End,
    }
    
    match buf[0] {
        CodeLookAt => {
            match bytes.read(&mut buf) {
                Ok(1) => (),
                _ => return Command::End,
            }
            
            let dx = i8_from_u8(buf[0]) as i32;
            
            match bytes.read(&mut buf) {
                Ok(1) => (),
                _ => return Command::End,
            }
            
            let dy = i8_from_u8(buf[0]) as i32;
            
            ///////////// Positive Y is north, positive X is east
             
            return Command::LookAt(East * dx + North * dy);
        },
        CodeMove => {
            match bytes.read(&mut buf) {
                Ok(1) => (),
                _ => return Command::End,
            }
            
            return match code_to_direction(buf[0]) {
                Some(direction) => Command::Move(direction),
                None => Command::Malformed,
            };
        },
        CodeDrill => {
            match bytes.read(&mut buf) {
                Ok(1) => (),
                _ => return Command::End,
            }
            
            return match code_to_direction(buf[0]) {
                Some(direction) => Command::Drill(direction),
                None => Command::Malformed,
            };
        },
        _ => return Command::Malformed,
    }
}


fn notify(notification: Notification) {
    
}


fn execute_command(mut area: Area, bot: Position, command: Command) {
    match command {
        Command::LookAt(offset) => notify(Notification::YouSee(area.type_at(bot + offset))),
        Command::Move(direction) => area.bot_go(bot, direction),
        Command::Drill(direction) => area.bot_drill(bot, direction),
        Command::Malformed | Command::End => area.remove(bot),
    }
}


#[cfg(test)]
mod tests {
    use super::{Command, parse_next, i8_into_u8};
    use space::*;
    use space::Direction::*;
    use entity::*;
    use std::io::Write;
    
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
}

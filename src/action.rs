use space::*;
use space::Direction::*;
use entity::*;
use std::io::Read;
use std::mem::transmute;


#[derive(Copy, Clone, PartialEq, Debug)]
enum Command {
    LookAt(Offset),
    Move(Direction),
    Drill(Direction),
    Malformed,
    End,
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
        1 => {
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
        _ => return Command::Malformed,
    }
}



#[cfg(test)]
mod tests {
    use super::{Command, parse_next, i8_into_u8};
    use space::*;
    use space::Direction::*;
    use entity::*;
    
    extern crate memstream;
    use self::memstream::*;
    use std::io::Write;
    
                        
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
    }
}

use std::ops::{Add, Shr};


pub enum Direction {
    North,
    East,
    South,
    West
}


#[derive(PartialEq, Debug)]
pub struct Offset {
    x: i32,
    y: i32,
}
impl Offset {
    fn new(x: i32, y: i32) -> Offset {
        Offset {
            x: x,
            y: y,
        }
    }
}
impl From<Direction> for Offset {
    fn from(other: Direction) -> Offset {
        match other {
            Direction::North => Offset { x: 0, y: -1 },
            Direction::East => Offset { x: 1, y: 0 },
            Direction::South => Offset { x: 0, y: 1 },
            Direction::West => Offset { x: -1, y: 0 },
        }
    }
}
impl From<(i32, i32)> for Offset {
    fn from(other: (i32, i32)) -> Self {
        let (x, y) = other;
        Offset { x: x, y: y }
    }
}

impl Add<Position> for Offset {
    type Output = Position;
    
    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Add<Position> for (i32, i32) {
    type Output = Position;
    
    fn add(self, other: Position) -> Position {
        let offset: Offset = self.into();
        Position {
            x: offset.x + other.x,
            y: offset.y + other.y,
        }
    }
}


#[derive(PartialEq, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
}
impl Shr<Position> for Position {
    type Output = Offset;
    
    fn shr(self, other: Position) -> Offset {
        Offset {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}


impl<R: Into<Offset>+Sized> Add<R> for Position {
    type Output = Position;
    
    fn add(self, other: R) -> Position {
        let offset: Offset = other.into();
        Position {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}


#[test]
fn add_position_offset() {
    assert_eq!(Offset::new(0, 0) + Position::new(0, 0), Position::new(0, 0));
    assert_eq!(Offset::new(1, 0) + Position::new(0, 0), Position::new(1, 0));
    assert_eq!(Offset::new(-1, 0) + Position::new(2, 1), Position::new(1, 1));
    assert_eq!(Position::new(-1, 1) + Offset::new(-1, 1), Position::new(-2, 2));
}

#[test]
fn offset_from_positions() {
    assert_eq!(Position::new(-1, 1) >> Position::new(-2, 2), Offset::new(-1, 1));
    assert_eq!(Position::new(2, 1) >> Position::new(2, 2), Offset::new(0, 1));
    assert_eq!(Position::new(0, 0) >> Position::new(0, 3), Offset::new(0, 3));
}

#[test]
fn add_direction_position() {
    assert_eq!(Position::new(0, 0) + Direction::North + Direction::South, Position::new(0, 0));
    assert_eq!(Position::new(0, 0) + Direction::East + Direction::West, Position::new(0, 0));
}

#[test]
fn add_tuple_position() {
    assert_eq!(Position::new(0, 0) + (0, 0), Position::new(0, 0));
    assert_eq!(Position::new(0, 0) + (1, 0), Position::new(1, 0));
    assert_eq!(Position::new(2, 1) + (-1, 0), Position::new(1, 1));
    assert_eq!(Position::new(-1, 1) + (-1, 1), Position::new(-2, 2));
}

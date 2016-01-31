use std::ops::{Add, Shr, Mul};
use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Mul<i32> for Direction {
    type Output = Offset;
    
    fn mul(self, other: i32) -> Offset {
        let offset: Offset = self.into();
        offset * other
    }
}
impl Mul<Direction> for i32 {
    type Output = Offset;
    
    fn mul(self, other: Direction) -> Offset {
        other * self
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
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
    fn from(other: Direction) -> Self {
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
impl Add<Offset> for Offset {
    type Output = Offset;
    
    fn add(self, other: Offset) -> Offset {
        Offset {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Mul<i32> for Offset {
    type Output = Offset;
    
    fn mul(self, other: i32) -> Offset {
        Offset {
            x: self.x * other,
            y: self.y * other,
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


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
    }
    fn zero() -> Position {
        Position { x: 0, y: 0 }
    }
}
impl From<(i32, i32)> for Position {
    fn from(other: (i32, i32)) -> Self {
        let (x, y) = other;
        Position { x: x, y: y }
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
        let offset = other.into();
        Position {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    topleft: Position,
    size: Offset,
}
impl Rectangle {
    pub fn xywh(topleft: Position, size: Offset) -> Rectangle {
        Rectangle {
            topleft: topleft,
            size: size,
        }
    }
    
    pub fn wh(size: Offset) -> Rectangle {
        Rectangle {
            topleft: Position {
                x: 0,
                y: 0,
            },
            size: size,
        }
    }
    
    pub fn corners(a: Position, b: Position) -> Rectangle {
        let topleft = Position {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
        };
        let bottomright = Position {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
        };
        Rectangle::xywh(topleft, topleft >> bottomright + Direction::South + Direction::East)
    }
    pub fn corner_offsets(a: Offset, b: Offset) -> Rectangle {
        Rectangle::corners(a + Position::zero(), b + Position::zero())
    }
    
    pub fn contains(self, position: Position) -> bool {
        let bottomright = self.topleft + self.size;
        ((position.x >= self.topleft.x) != (position.x >= bottomright.x)) &&
            ((position.y >= self.topleft.y) != (position.y >= bottomright.y))
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
fn add_offsets() {
    assert_eq!(Offset::new(0, 0) + Offset::new(0, 0), Offset::new(0, 0));
    assert_eq!(Offset::new(1, 0) + Offset::new(0, 0), Offset::new(1, 0));
    assert_eq!(Offset::new(-1, 0) + Offset::new(2, 1), Offset::new(1, 1));
    assert_eq!(Offset::new(-1, 1) + Offset::new(-1, 1), Offset::new(-2, 2));
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

#[test]
fn offset_multiplication() {
    assert_eq!(Offset::new(1, 1) * 3, Offset::new(3, 3));
    assert_eq!(Offset::new(0, -1) * 4, Offset::new(0, -4));
    assert_eq!(Direction::North * 2, Offset::new(0, -2));
}

#[test]
fn offset_from_directions() {
    assert_eq!(Direction::North * 3 + Direction::East * 2, Offset::new(2, -3));
    assert_eq!(Direction::North * -3 + Direction::East * 2, Offset::new(2, 3));
    assert_eq!(Direction::South * 3 + Direction::East * 2, Offset::new(2, 3));
}

#[test]
fn containment() {
    assert!(Rectangle::wh(Offset::new(2, 2)).contains(Position::new(0, 0)));
    assert!(Rectangle::wh(Offset::new(2, 2)).contains(Position::new(1, 1)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(1, 2)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(2, 2)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(2, 0)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(-1, 0)));
    
    let rec = Rectangle::corner_offsets(Direction::North * 2 + Direction::West * 2,
                                        Direction::South * 4 + Direction::East * 3);
    assert!(rec.contains(Position::zero() + Direction::North * 2));
    assert!(rec.contains(Position::zero() + Direction::North * 2 + Direction::West * 2));
    assert!(rec.contains(Position::zero() + Direction::North * 2 + Direction::East * 3));
}

#[test]
fn direction_equality() {
    assert_eq!(Direction::East, Direction::East);
    assert!(Direction::East != Direction::West);
}

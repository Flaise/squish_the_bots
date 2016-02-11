use std::ops::{Add, Shr, Mul};
use std::cmp::{max, min};
use self::Direction::*;


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
    pub fn zero() -> Offset {
        Offset { x: 0, y: 0 }
    }
}
impl From<Direction> for Offset {
    fn from(other: Direction) -> Self {
        match other {
            North => Offset { x: 0, y: -1 },
            East => Offset { x: 1, y: 0 },
            South => Offset { x: 0, y: 1 },
            West => Offset { x: -1, y: 0 },
        }
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
impl Mul<i32> for Offset {
    type Output = Offset;
    
    fn mul(self, other: i32) -> Offset {
        Offset {
            x: self.x * other,
            y: self.y * other,
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
    pub fn zero() -> Position {
        Position { x: 0, y: 0 }
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
impl<R: Into<Offset>+Sized> Add<R> for Offset {
    type Output = Offset;
    
    fn add(self, other: R) -> Offset {
        let offset = other.into();
        Offset {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}
impl<R: Into<Offset>+Sized> Add<R> for Direction {
    type Output = Offset;
    
    fn add(self, other: R) -> Offset {
        let a: Offset = self.into();
        let b: Offset = other.into();
        a + b
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
            topleft: Position::zero(),
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
        Rectangle::xywh(topleft, topleft >> bottomright + South + East)
    }
    
    pub fn corner_offsets(a: Offset, b: Offset) -> Rectangle {
        Rectangle::corners(a + Position::zero(), b + Position::zero())
    }
    
    pub fn contains(self, position: Position) -> bool {
        let mut p = position;
        let mut bottomright = self.topleft + self.size;
        if bottomright.x < self.topleft.x {
            p.x -= 1;
        }
        if bottomright.y < self.topleft.y {
            p.y -= 1;
        }
        ((p.x >= self.topleft.x) != (p.x >= bottomright.x)) &&
            ((p.y >= self.topleft.y) != (p.y >= bottomright.y))
    }
    
    pub fn area(self) -> i32 {
        (self.size.x * self.size.y).abs()
    }
}
impl IntoIterator for Rectangle {
    type Item = Position;
    type IntoIter = RectangleContents;
    
    fn into_iter(self) -> Self::IntoIter {
        RectangleContents {
            subject: self,
            cursor: Offset::zero(),
        }
    }
}

pub struct RectangleContents {
    subject: Rectangle,
    cursor: Offset,
}
impl Iterator for RectangleContents {
    type Item = Position;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.y.abs() >= self.subject.size.y.abs() {
            return None;
        }
        let result = self.subject.topleft + self.cursor;
        self.cursor.x += self.subject.size.x.signum();
        if self.cursor.x.abs() >= self.subject.size.x.abs() {
            self.cursor.y += self.subject.size.y.signum();
            self.cursor.x = 0;
        }
        Some(result)
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
fn add_directions() {
    let north_offset: Offset = North.into();
    let east_offset: Offset = East.into();
    
    assert_eq!(North + East, north_offset + east_offset);
}

#[test]
fn add_direction_offset() {
    assert_eq!(North + Offset::zero(), Offset::new(0, -1));
    assert_eq!(Offset::zero() + North, Offset::new(0, -1));
}

#[test]
fn offset_from_positions() {
    assert_eq!(Position::new(-1, 1) >> Position::new(-2, 2), Offset::new(-1, 1));
    assert_eq!(Position::new(2, 1) >> Position::new(2, 2), Offset::new(0, 1));
    assert_eq!(Position::new(0, 0) >> Position::new(0, 3), Offset::new(0, 3));
}

#[test]
fn add_direction_position() {
    assert_eq!(Position::new(0, 0) + North + South, Position::new(0, 0));
    assert_eq!(Position::new(0, 0) + East + West, Position::new(0, 0));
}

#[test]
fn offset_multiplication() {
    assert_eq!(Offset::new(1, 1) * 3, Offset::new(3, 3));
    assert_eq!(Offset::new(0, -1) * 4, Offset::new(0, -4));
    assert_eq!(North * 2, Offset::new(0, -2));
}

#[test]
fn offset_from_directions() {
    assert_eq!(North * 3 + East * 2, Offset::new(2, -3));
    assert_eq!(North * -3 + East * 2, Offset::new(2, 3));
    assert_eq!(South * 3 + East * 2, Offset::new(2, 3));
}

#[test]
fn containment() {
    assert!(Rectangle::wh(Offset::new(1, 1)).contains(Position::new(0, 0)));
    assert!(Rectangle::wh(Offset::new(2, 2)).contains(Position::new(0, 0)));
    assert!(Rectangle::wh(Offset::new(2, 2)).contains(Position::new(1, 1)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(1, 2)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(2, 2)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(2, 0)));
    assert!(!Rectangle::wh(Offset::new(2, 2)).contains(Position::new(-1, 0)));
    
    assert!(Rectangle::wh(Offset::new(-2, -2)).contains(Position::new(0, 0)));
    assert!(Rectangle::wh(Offset::new(-2, -2)).contains(Position::new(-1, 0)));
    assert!(!Rectangle::wh(Offset::new(-2, -2)).contains(Position::new(-2, 0)));
    
    let rec = Rectangle::corner_offsets(North * 2 + West * 2, South * 4 + East * 3);
    assert!(rec.contains(Position::zero() + North * 2));
    assert!(rec.contains(Position::zero() + North * 2 + West * 2));
    assert!(rec.contains(Position::zero() + North * 2 + East * 3));
    
    let rec = Rectangle::corner_offsets(South * 4 + East * 3, North * 2 + West * 2);
    assert!(rec.contains(Position::zero() + North * 2));
    assert!(rec.contains(Position::zero() + North * 2 + West * 2));
    assert!(rec.contains(Position::zero() + North * 2 + East * 3));
}

#[test]
fn direction_equality() {
    assert_eq!(East, East);
    assert!(East != West);
}

#[test]
fn rectangle_iteration() {
    let rect = Rectangle::wh(Offset::new(1, 1));
    assert_eq!(rect.into_iter().collect::<Vec<_>>(), vec![Position::zero()]);
    
    let rect = Rectangle::wh(Offset::new(2, 1));
    assert_eq!(rect.into_iter().collect::<Vec<_>>(),
               vec![Position::zero(), Position::zero() + East]);
    
    let rect = Rectangle::wh(Offset::new(1, 2));
    assert_eq!(rect.into_iter().collect::<Vec<_>>(),
               vec![Position::zero(), Position::zero() + South]);
    
    let rect = Rectangle::wh(Offset::new(2, 2));
    assert_eq!(rect.into_iter().collect::<Vec<_>>(),
               vec![Position::zero(), Position::zero() + East, Position::zero() + South,
                    Position::zero() + South + East]);
}

#[test]
fn rectangle_iteration_2() {
    let position = Position::zero() + North * 5 + West * 2;
    let rect = Rectangle::xywh(position, East * 20 + South * 19);
    for position in rect {
        assert!(rect.contains(position));
    }
    
    assert_eq!(rect.topleft, position); // rect should not be consumed by iterator
}

#[test]
fn area() {
    for rect in vec![
        Rectangle::wh(East * 20 + South * 19),
        Rectangle::wh(East * 10 + South * 1),
        Rectangle::wh(West * 4 + South * 17),
        Rectangle::wh(West * 4 + South * -3),
        Rectangle::wh(East * -4 + North * 17),
    ] {
        assert_eq!(rect.area(), rect.into_iter().collect::<Vec<_>>().len() as i32);
    }
}

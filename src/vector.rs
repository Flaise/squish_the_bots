use std::ops::{Add, Sub, Shr, Mul};
use std::cmp::{max, min};


#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vector2<A> {
    pub x: A,
    pub y: A,
}

pub fn vec_min<R: Ord, S: Into<Vector2<R>>>(a: S, b: S) -> Vector2<R> {
    let a = a.into();
    let b = b.into();
    Vector2 {
        x: min(a.x, b.x),
        y: min(a.y, b.y),
    }
}
pub fn vec_max<R: Ord, S: Into<Vector2<R>>>(a: S, b: S) -> Vector2<R> {
    let a = a.into();
    let b = b.into();
    Vector2 {
        x: max(a.x, b.x),
        y: max(a.y, b.y),
    }
}

impl<A: Add<B, Output=C>, B, C> Add<Vector2<B>> for Vector2<A> {
    type Output = Vector2<C>;
    
    fn add(self, other: Vector2<B>) -> Vector2<C> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<A: Sub<B, Output=C>, B, C> Sub<Vector2<B>> for Vector2<A> {
    type Output = Vector2<C>;
    
    fn sub(self, other: Vector2<B>) -> Vector2<C> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<A: Mul<B, Output=C>, B: Copy, C> Mul<B> for Vector2<A> {
    type Output = Vector2<C>;
    
    fn mul(self, other: B) -> Vector2<C> {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl<A, B: Sub<A, Output=C>, C> Shr<Vector2<B>> for Vector2<A> {
    type Output = Vector2<C>;
    
    fn shr(self, other: Vector2<B>) -> Vector2<C> {
        Vector2 {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

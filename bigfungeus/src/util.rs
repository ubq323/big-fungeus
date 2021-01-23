use crate::FungeCell;
use std::ops::{Add,AddAssign,Mul};

#[derive(Debug,Clone,Default,Copy,Hash)]
pub struct Vec2D {
    pub x: FungeCell,
    pub y: FungeCell,
}

impl Vec2D {
    pub const fn zero() -> Vec2D {
        Vec2D { x:0, y:0 }
    }
    pub const fn new(x:FungeCell,y:FungeCell) -> Vec2D {
        Vec2D {x,y}
    }
    pub fn reflect(&self) -> Vec2D {
        Vec2D::new(-self.x,-self.y)
    }
    pub fn rotl(&self) -> Vec2D {
        // +y is down
        Vec2D::new(self.y,-self.x)
    }
    pub fn rotr(&self) -> Vec2D {
        Vec2D::new(-self.y,self.x)
    }
}
impl Add<Vec2D> for Vec2D {
    type Output = Self;
    fn add(self,other:Self) -> Self {
        Vec2D::new(self.x+other.x,self.y+other.y)
    }
}
impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self,other:Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl Mul<FungeCell> for Vec2D {
    type Output = Self;
    fn mul(self,scalar:FungeCell) -> Self {
        Vec2D::new(self.x*scalar,self.y*scalar)
    }
}

impl From<Vec2D> for (FungeCell,FungeCell) {
    fn from(v: Vec2D) -> Self {
        (v.x,v.y)
    }
}
impl From<(FungeCell,FungeCell)> for Vec2D {
    fn from(v: (FungeCell,FungeCell)) -> Self {
        Vec2D::new(v.0, v.1)
    }
}
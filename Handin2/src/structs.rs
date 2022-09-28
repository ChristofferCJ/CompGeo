use crate::structs::Side::{BOTTOM, INTERSECT, TOP};

#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Line {
    pub start: Point,
    pub end: Point
}

#[derive(PartialEq)]
pub enum Side {
    TOP,
    BOTTOM,
    INTERSECT
}

impl Line {
    pub fn check_side(&self, p: &Point) -> Side {
        let sign = ((p.x - self.start.x) 
            * (self.end.y - self.start.y))
            - ((p.y - self.start.y)
            * (self.end.x - self.start.x));
        
            if sign > 0.0 {
                return TOP
            }
            else if sign < 0.0 {
                return BOTTOM
            }
            else {
                return INTERSECT
            }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn check_side(sl: Point, el: Point, p: Point) -> Side {
    let sign = ((p.x - sl.x)
        * (el.y - sl.y))
        - ((p.y - sl.y)
        * (el.x - sl.x));

    if sign > 0.0 {
        return Side::BOTTOM;
    } else if sign < 0.0 {
        return Side::TOP;
    } else {
        return Side::INTERSECT;
    }
}

#[derive(Debug, PartialEq)]
pub enum Side {
    TOP,
    BOTTOM,
    INTERSECT,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn check_side(&self, p: &Point) -> Side {
        let sign = ((p.x - self.start.x)
            * (self.end.y - self.start.y))
            - ((p.y - self.start.y)
            * (self.end.x - self.start.x));

        if sign > 0.0 {
            return Side::BOTTOM;
        } else if sign < 0.0 {
            return Side::TOP;
        } else {
            return Side::INTERSECT;
        }
    }
}
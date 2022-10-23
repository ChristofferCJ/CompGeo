#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
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

pub enum Direction {
    LEFT,
    RIGHT,
    UPPER_TANGENT,
    LOWER_TANGENT,
    OTHER
}

#[derive(Debug, Copy, Clone)]
pub enum Hull {
    UPPER,
    LOWER
}

#[derive(Copy, Clone, Debug)]
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

    pub fn determine_direction_for_upper_hull(&self, lp: &Point, rp: &Point) -> Direction {
        return match (self.check_side(lp), self.check_side(rp)) {
            (Side::TOP, Side::BOTTOM) => Direction::LEFT,
            (Side::BOTTOM, Side::TOP) => Direction::RIGHT,
            (Side::TOP, Side::TOP) => Direction::UPPER_TANGENT,
            (Side::INTERSECT, _) => Direction::UPPER_TANGENT,
            (_, Side::INTERSECT) => Direction::UPPER_TANGENT,
            _ => panic!("idk")
        };
    }

    pub fn determine_direction_for_lower_hull(&self, lp: &Point, rp: &Point) -> Direction {
        return match (self.check_side(lp), self.check_side(rp)) {
            (Side::TOP, Side::BOTTOM) => Direction::RIGHT,
            (Side::BOTTOM, Side::TOP) => Direction::LEFT,
            (Side::BOTTOM, Side::BOTTOM) => Direction::LOWER_TANGENT,
            (Side::INTERSECT, _) => Direction::LOWER_TANGENT,
            (_, Side::INTERSECT) => Direction::LOWER_TANGENT,
            (a, b) => {
                println!("A: {:?}", a);
                println!("B: {:?}", b);
                panic!("idk")
            }
        };
    } 

    pub fn determine_direction(&self, lp: &Point, rp: &Point, hull: &Hull) -> Direction {
        match hull {
            Hull::UPPER => {
                match (self.check_side(lp), self.check_side(rp)) {
                    (Side::TOP, Side::TOP) => Direction::OTHER,
                    (Side::TOP, Side::BOTTOM) => return Direction::LEFT,
                    (Side::TOP, Side::INTERSECT) => panic!("Upper hull is not an upper hull"),
                    (Side::BOTTOM, Side::TOP) => return Direction::RIGHT,
                    (Side::BOTTOM, Side::BOTTOM) => return Direction::UPPER_TANGENT,
                    (Side::BOTTOM, Side::INTERSECT) => return Direction::UPPER_TANGENT,
                    (Side::INTERSECT, Side::TOP) => panic!("Upper hull is not an upper hull"),
                    (Side::INTERSECT, Side::BOTTOM) => return Direction::UPPER_TANGENT,
                    (Side::INTERSECT, Side::INTERSECT) => return Direction::UPPER_TANGENT,
                }
            },
            Hull::LOWER => {
                match (self.check_side(lp), self.check_side(rp)) {
                    (Side::TOP, Side::TOP) => return Direction::LOWER_TANGENT,
                    (Side::TOP, Side::BOTTOM) => return Direction::RIGHT,
                    (Side::TOP, Side::INTERSECT) => return Direction::LOWER_TANGENT,
                    (Side::BOTTOM, Side::TOP) => return Direction::LEFT,
                    (Side::BOTTOM, Side::BOTTOM) => return Direction::OTHER,
                    (Side::BOTTOM, Side::INTERSECT) => panic!("Lower hull is not a lower hull"),
                    (Side::INTERSECT, Side::TOP) => return Direction::LOWER_TANGENT,
                    (Side::INTERSECT, Side::BOTTOM) => panic!("Lower hull is not a lower hull"),
                    (Side::INTERSECT, Side::INTERSECT) => return Direction::LOWER_TANGENT,
                }
            }
        }
    }
}
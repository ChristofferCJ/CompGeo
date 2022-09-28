struct Point {
    x: f32,
    y: f32
}

struct Line {
    start: Point,
    end: Point
}

enum Side {
    TOP,
    BOTTOM,
    INTERSECT
}

impl Line {
    fn check_side(&self, p: Point) -> Side {
        let sign = ((p.x - self.start.x) 
            * (self.end.y - self.start.y))
            - ((p.y - self.start.y)
            * (self.end.x - self.start.x));
        
            if sign > 0 {
                return TOP
            }
            else if sign < 0 {
                return BOTTOM
            }
            else {
                return INTERSECT
            }
    }
}
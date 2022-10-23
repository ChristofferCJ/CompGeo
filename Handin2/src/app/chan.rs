use crate::app::{structs::{Hull, Side, Direction}, graham::{find_hull, merge_hulls}};

use super::structs::{Point, Line};

pub fn chan(points: &Vec<Point>) -> Option<Vec<Point>> {
    let uh = hull(&points, &Hull::UPPER);
    let lh = hull(&points, &Hull::LOWER);

    if uh.is_none() && lh.is_none() { return None; }

    let hull = merge_hulls(uh.unwrap(), lh.unwrap());
    return Some(hull);
}

fn hull(points: &Vec<Point>, hull: &Hull) -> Option<Vec<Point>> {
    let n = points.len() as f64;
    let iters = (n.log2().log2().ceil() as u32) + 2;
    for i in 1..=iters {
        let size = 2u32.pow(2u32).pow(i);
        let guess = hull_with_size(points, hull, size);
        if guess.is_some() { return guess }
    }
    return None;
}

fn hull_with_size(points: &Vec<Point>, hull: &Hull, size: u32) -> Option<Vec<Point>> {
    let partitions = partition_points(points, size);
    let mut hulls = find_hulls(partitions, *hull);
    let mut h: Vec<Point> = Vec::new();
    let mut p = left_most_point(points);
    let mut l = init_ray(&p, hull);
    let rightmost = rightmost_point(points);
    for _ in 0..size {
        h.push(p);
        if p.x >= rightmost.x { break; }
        let mut tangents: Vec<Line> = Vec::new();
        for i in 0..hulls.len() {
            if hulls[i].len() > 0 {
                let tangent = compute_tangent(&hulls[i], &p, hull);
                tangents.push(tangent);
            }
        }
        l = smallest_tangent(&tangents, l, hull);
        p = l.end;
        remove_left_points(&mut hulls, &p);
    }
    if p.x >= rightmost.x {
        return Some(h);
    } else {
        return None
    }
}

fn remove_left_points(hulls: &mut Vec<Vec<Point>>, p: &Point) {
    for hull in hulls {
        while hull.len() > 0 && hull.last().unwrap().x <= p.x {
            hull.pop();
        }
    }
}

fn smallest_tangent(tangents: &Vec<Line>, l: Line, hull: &Hull) -> Line {
    let l_slope = (l.end.y - l.start.y) / (l.end.x - l.start.x);
    let mut res = tangents.first().unwrap();
    let mut slope = (res.end.y - res.start.y) / (res.end.x - res.start.x);
    let mut angle = l_slope - slope;
    for t in tangents {
        slope = (t.end.y - t.start.y) / (t.end.x - t.start.x);
        match hull {
            Hull::UPPER => {
                if angle > l_slope - slope {
                    angle = l_slope - slope;
                    res = t;
                }
            },
            Hull::LOWER => {
                if angle < l_slope - slope {
                    angle = l_slope - slope;
                    res = t;
                }
            }
        }
    }
    return *res;
}


fn partition_points(points: &Vec<Point>, partition_size: u32) -> Vec<Vec<Point>> {
    return points.chunks(partition_size as usize)
        .map(|c| c.into())
        .collect();
}

fn find_hulls(partitions: Vec<Vec<Point>>, hull: Hull) -> Vec<Vec<Point>> {
    let mut res: Vec<Vec<Point>> = Vec::new();
    for mut partition in partitions {
        partition.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        if partition.len() < 3 {
            res.push(partition);
            continue;
        }
        match hull {
            Hull::UPPER => {
                let uh = find_hull(&partition, Side::TOP);
                res.push(uh);
                continue;
            },
            Hull::LOWER => {
                let lh = find_hull(&partition, Side::BOTTOM);
                res.push(lh);
                continue;
            }
        }
    }
    for arr in &mut res {
        arr.reverse();
    }
    return res;
}

fn left_most_point(points: &Vec<Point>) -> Point {
    if points.len() == 0 { panic!("List was empty"); }
    let mut res = points.first().unwrap();
    for point in points {
        if point.x < res.x { res = point; }
    }
    return *res;
}

fn rightmost_point(points: &Vec<Point>) -> Point {
    let mut p = points.first().unwrap();
    for point in points {
        if point.x > p.x {
            p = point;
        }
    }
    return p.clone();
}

fn init_ray(point: &Point, hull: &Hull) -> Line {
    match hull {
        Hull::UPPER => {
            let end = Point { x: point.x + 1f64, y: point.y };
            return Line { start: *point, end: end }
        },
        Hull::LOWER => {
            let end = Point { x: point.x - 1f64, y: point.y };
            return Line { start: *point, end: end }
        }
    }
}

fn compute_tangent(points: &Vec<Point>, p: &Point, hull: &Hull) -> Line {
    if points.len() == 1 { return Line {start: *p, end: points[0]}; }
    let mut idx = points.len() / 2;
    loop {
        let line = Line { start: *p, end: *points.get(idx).unwrap() };
        let right = points.get(idx+1).or(points.first()).unwrap();
        let left = if idx == 0 { points.last().unwrap() } else { points.get(idx-1).unwrap() }; // this is very stupid
        match line.determine_direction(left, right, hull) {
            Direction::LEFT => {
                idx -= 1;
            },
            Direction::RIGHT => {
                idx += 1;
            },
            Direction::UPPER_TANGENT => {
                return line;
            },
            Direction::LOWER_TANGENT => {
                return line;
            },
            Direction::OTHER => {
                assert!(points.len() == 2);
                idx = 1 - idx;
            }
        }
    }
}
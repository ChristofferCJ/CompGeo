use crate::app::structs::{check_side, Side};
use crate::Point;

pub fn grahams_scan(mut points: Vec<Point>) -> Option<Vec<Point>> {
    if points.len() < 3 { return None; }
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let uh = find_hull(&points, Side::TOP);
    let lh = find_hull(&points, Side::BOTTOM);
    let hull = merge_hulls(uh, lh);
    return Some(hull);
}

pub fn print_hull(hull: &Vec<Point>, name: &str) {
    println!("{:?}", name);
    for point in hull.iter() {
        println!("{:?}", point);
    }
}

pub fn merge_hulls(uh: Vec<Point>, mut lh: Vec<Point>) -> Vec<Point> {
    let mut hull = Vec::<Point>::new();
    for point in uh.into_iter() {
        hull.push(point);
    }
    // remove first and last element of lower hull, as they are contained in upper hull
    lh.remove(0);
    lh.pop();
    for point in lh.into_iter().rev() {
        hull.push(point);
    }
    return hull;
}

pub fn find_hull(points: &Vec<Point>, side: Side) -> Vec<Point> {
    // assumes the points are sorted and there are at least 3 points
    let mut hull = Vec::<Point>::new();
    hull.push(points[0]);
    hull.push(points[1]);
    let mut s = 1;
    for i in 2..points.len() {
        while s >= 1 && check_side(hull[s-1], hull[s], points[i]) == side {
            hull.pop();
            s -= 1;
        }
        hull.push(points[i]);
        s += 1;
    }
    return hull;
}
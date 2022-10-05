use crate::app::structs::*;

pub fn gift_wrapping(points: &Vec<Point>) -> Vec<Point> {

    let mut point_on_hull = left_most_point(points);

    let mut hull = Vec::new();
    loop {
        hull.push(point_on_hull.clone());
        let mut endpoint = points.first().unwrap();

        for j in 0..points.len() {
            let latest_on_hull = hull.last().unwrap();
            let point_j = points[j];
            let line = Line {
                start: latest_on_hull.clone(),
                end: endpoint.clone(),
            };
            if endpoint == point_on_hull || line.check_side(&point_j) == Side::TOP {
                endpoint = &points[j];
            }
        }
        point_on_hull = endpoint;

        if endpoint == hull.first().unwrap() {
            break;
        }
    }

    return hull;
}

fn left_most_point(points: &Vec<Point>) -> &Point {
    let mut left = points.first().unwrap();
    for p in points {
        if p.x < left.x {
            left = p;
        }
    }
    return left;
}
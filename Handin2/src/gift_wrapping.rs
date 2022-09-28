use crate::structs::*;

fn gift_wrapping(points: &Vec<Point>) -> Vec<Point> {

    let mut point_on_hull = left_most_point(points);

    let mut hull = Vec::new();
    loop {
        hull.push(point_on_hull.clone());
        let mut endpoint = points.first().unwrap();

        for j in 0..points.len() {
            let line = Line {
                start: points[j],
                end: endpoint.clone(),
            };
            //TODO check if on the left instead of sideness
            if endpoint == point_on_hull || line.check_side(endpoint) == Side::TOP {
                endpoint = &points[j];
            }
        }

        //TODO end condition
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
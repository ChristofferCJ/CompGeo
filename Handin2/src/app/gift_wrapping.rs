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
            if latest_on_hull == &point_j {
                continue;
            }

            let line = Line {
                start: latest_on_hull.clone(),
                end: endpoint.clone(),
            };
            let side = line.check_side(&point_j);
            if endpoint == point_on_hull || side == Side::TOP {
                endpoint = &points[j];
            }
            else if side == Side::INTERSECT {
                // Take the closest point if there is intersection
                // Fixes hull for points on curve, when precision is not high enough
                let dist_1 = (line.start.x - line.end.x).powi(2) +
                    (line.start.y - line.end.y).powi(2);
                let dist_2 = (line.start.x - point_j.x).powi(2) +
                    (line.start.y - point_j.y).powi(2);

                if dist_2 < dist_1 {
                    endpoint = &points[j];
                }
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
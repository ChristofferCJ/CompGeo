use graham::{grahams_scan, print_hull};
use structs::Point;

mod graham;
mod gift_wrapping;
mod structs;

fn main() {
    let points = vec![
        Point {x: 0.01, y: 0.0},
        Point {x: 0.02, y: 1.0},
        Point {x: 1.03, y: 1.0},
        Point {x: 1.04, y: 0.0},
        Point {x: 0.55, y: 0.5},
        Point {x: -0.5, y: 0.5},
        Point {x: 0.56, y: 0.5},
        Point {x: 0.57, y: 0.5},
    ];
    let result = grahams_scan(points);
    match result {
        Some(points) => {
            print_hull(&points, "Convex hull".to_owned());
        },
        None => { panic!("You fucked up") }
    }
}

use app::graham::{grahams_scan, print_hull};
use app::structs::Point;
use app::gift_wrapping::gift_wrapping;
use crate::tests::test_data::points_1;

mod app;
mod tests;

fn main() {

    let result = grahams_scan(points_1());
    match result {
        Some(points) => {
            print_hull(&points, "Convex hull(graham scan)");
        },
        None => { panic!("You fucked up") }
    }

    println!();

    let result = gift_wrapping(&points_1());
    print_hull(&result, "Convex hull(gift wrapping)");
}

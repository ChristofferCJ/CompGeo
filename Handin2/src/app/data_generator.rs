use std::f64::consts::PI;
use crate::Point;
use rand::prelude::*;
use rand::rngs::StdRng;

pub fn generate_points_square(amount: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::<Point>::new();

    loop {

        let x = rng.gen::<f64>() * 100.0;
        let y = rng.gen::<f64>() * 100.0;

        if points.iter().any(|&p| p.x == x || p.y == y) {
            continue
        }
        points.push(Point {x, y});

        if points.len() >= amount {
            return points;
        }
    }
}

pub fn generate_points_circle(amount: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::<Point>::new();

    loop {

        let r = 100.0 * rng.gen::<f64>().sqrt();
        let theta = rng.gen::<f64>() * 2.0 * PI;
        let x = r * theta.cos();
        let y = r * theta.sin();

        if points.iter().any(|&p| p.x == x || p.y == y) {
            continue
        }
        points.push(Point {x, y});

        if points.len() >= amount {
            return points;
        }
    }
}

pub fn generate_points_curve(amount: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::<Point>::new();

    loop {
        // Lower precision for x, so that there is enough precision space for y
        let x = (((rng.gen::<f32>() * 1000000.0) as i64) as f64) / 1000.0;
        let y = x*x;

        if points.iter().any(|&p| p.x == x || p.y == y) {
            continue
        }
        points.push(Point {x, y});

        if points.len() >= amount {
            return points;
        }
    }
}

use std::f32::consts::PI;
use crate::Point;
use rand::prelude::*;
use rand::rngs::StdRng;

pub fn generate_points_square(amount: usize, seed: u64) -> Vec<Point> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut points = Vec::<Point>::new();

    loop {

        let x = rng.gen::<f32>() * 100.0;
        let y = rng.gen::<f32>() * 100.0;

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

        let r = 100.0 * rng.gen::<f32>().sqrt();
        let theta = rng.gen::<f32>() * 2.0 * PI;
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

        let x = rng.gen::<f32>() * 100.0;
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

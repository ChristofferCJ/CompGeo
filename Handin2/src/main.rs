use std::path::PathBuf;
use std::time::{Duration, Instant};
use app::graham::{grahams_scan, print_hull};
use app::structs::Point;
use app::gift_wrapping::gift_wrapping;
use crate::app::data_generator::generate_points_square;
use crate::tests::test_data::points_1;
use clap;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::app::benchmark::{benchmark_algorithms, BenchmarkResult};
use crate::app::chan::chan;

mod app;
mod tests;


#[derive(Clone, ValueEnum)]
enum Action {
    Benchmark,
    GrahamsScan,
    GiftWrapping,
    ChansAlgorithm,
}

#[derive(Parser)]
struct Cli {
    action: Action,
    input: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let points_result = read_points_from_file(&args.input);

    match points_result {
        Ok(points) => {
            handle_action(&args,&points);
        }
        Err(err) => {
            println!("Failed to read the file");
            println!("{err}");
        }
    }
}

fn handle_action(args: &Cli, data: &Vec<Point>) {
    match args.action {
        Action::Benchmark => {
            print_benchmark(data)
        }
        Action::GrahamsScan => {
            let result = grahams_scan(data).unwrap();
            print_points(&result);
        }
        Action::GiftWrapping => {
            let result = gift_wrapping(data);
            print_points(&result);
        }
        Action::ChansAlgorithm => {
            let result = chan(data).unwrap();
            print_points(&result)
        }
    }
}

fn print_benchmark(points: &Vec<Point>) {
    let result = benchmark_algorithms(points);
    match result {
        BenchmarkResult::Success {
            graham_scan,
            gift_wrapping,
            chan,
        } => {
            println!("OK gs={graham_scan} gw={gift_wrapping} ch={chan}")
        }
        BenchmarkResult::Error { msg } => {
            println!("ERROR {msg}")
        }
    }
}

fn print_points(points: &Vec<Point>) {
    for p in points {
        println!("{} {}", p.x, p.y);
    }
}

fn read_points_from_file(file_path: &PathBuf) -> std::io::Result<Vec<Point>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut points = Vec::new();
    for line in reader.lines() {
        let line_str = line?;
        let splitted = line_str.split(" ").collect::<Vec<&str>>();
        let x = splitted[0];
        let y = splitted[1];

        points.push(Point {
            x: x.parse::<f64>().unwrap(),
            y: y.parse::<f64>().unwrap(),
        })
    }

    return Ok(points);
}



fn test_elapsed_time() {
    let result = grahams_scan(&points_1());
    match result {
        Some(points) => {
            print_hull(&points, "Convex hull(graham scan)");
        }
        None => { panic!("You fucked up") }
    }

    println!();

    let result = gift_wrapping(&points_1());
    print_hull(&result, "Convex hull(gift wrapping)");

    println!("---------------");
    for _ in 0..10 {
        let result_1 = test_1().as_micros();
        println!("test 1: {result_1}");

        let result_2 = test_2().as_micros();
        println!("test 2: {result_2}");

        let result_3 = test_3().as_micros();
        println!("test 3: {result_3}");

        println!("---------------");
    }
}

fn test_1() -> Duration {
    let current = Instant::now();
    return current.elapsed();
}

fn test_2() -> Duration {
    let data = generate_points_square(1000, 0);
    let current = Instant::now();
    grahams_scan(&data);
    return current.elapsed();
}

fn test_3() -> Duration {
    let data = generate_points_square(1000, 0);
    let current = Instant::now();
    gift_wrapping(&data);
    return current.elapsed();
}
use std::time::Instant;
use crate::{chan, gift_wrapping, grahams_scan, Point};

pub enum BenchmarkResult {
    Success {
        graham_scan: u128,
        gift_wrapping: u128,
        chan: u128,
    },
    Error {
        msg: String,
    }
}

pub fn benchmark_algorithms(data: &Vec<Point>) -> BenchmarkResult {

    let (grahams_time, grahams_result) = measure_time(|| {
        grahams_scan(&data).unwrap()
    });
    let (gift_time, gift_result) = measure_time(|| {
        gift_wrapping(&data)
    });
    let (chan_time, chan_result) = measure_time(|| {
        chan(&data).unwrap()
    });

    if gift_result == grahams_result && gift_result == chan_result {
        return BenchmarkResult::Success {
            graham_scan: grahams_time,
            gift_wrapping: gift_time,
            chan: chan_time,
        }
    }

    return BenchmarkResult::Error {
        msg: "Outputs are not the same".to_string()
    };
}

fn measure_time<F, T>(f: F) -> (u128, T) where
 F: Fn() -> T {

    let current = Instant::now();
    let result = f();
    return (current.elapsed().as_micros(), result);
}
use crate::solution::Solution;
use rand::rngs::StdRng;
use rand::Rng;
use std::time::{Duration, Instant};

pub fn randomize_by_swaps(solution: &mut Solution, rng: &mut StdRng) {
    shuffle(&mut solution.order, rng)
}

pub fn generate_unique_duplet(max: usize, rng: &mut StdRng) -> (usize, usize) {
    let i = rng.gen_range(0..max);
    let j = (rng.gen_range(0..max - 1) + i + 1) % max;
    (i, j)
}

pub fn shuffle<T: Into<u32>>(vector: &mut Vec<T>, rng: &mut StdRng) {
    let size = vector.len();
    for i in 0..size {
        let j = rng.gen_range(i..size);
        vector.swap(i, j);
    }
}

pub fn generate_decision(probability: f64, rng: &mut StdRng) -> bool {
    if probability >= 1.0 {
        return true;
    } else if probability <= 0.0 {
        return false;
    }
    
    let sampled = rng.gen_range(0..i32::MAX);
    let threshold = (probability * i32::MAX as f64) as i32;
    sampled < threshold
}

pub fn measure_execution_time<F: FnMut()>(mut f: F) -> f64 {
    let mut total_duration = Duration::new(0, 0);
    let mut iterations = 0;
    let start = Instant::now();

    while total_duration.as_nanos() < 10 || iterations < 10 {
        f();
        total_duration = start.elapsed();
        iterations += 1;
    }

    (total_duration.as_nanos() as f64) / (iterations as f64)
}

pub fn humanize_time(time: f64) -> String {
    if time < 1_000.0 {
        format!("{:.2} ns", time)
    } else if time < 1_000_000.0 {
        format!("{:.2} us", time / 1_000.0)
    } else if time < 1_000_000_000.0 {
        format!("{:.2} ms", time / 1_000_000.0)
    } else {
        format!("{:.2} s", time / 1_000_000_000.0)
    }
}

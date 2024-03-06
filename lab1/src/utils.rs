use crate::solution::Solution;
use std::time::{Duration, Instant};
use rand::Rng;

pub fn randomize_by_swaps(original_solution: &Solution) -> Solution {
    let mut solution = original_solution.clone();

    for i in 0..solution.dimension {
        let j = rand::thread_rng().gen_range(i..solution.dimension);
        solution.order.swap(i, j);
    }
    solution
}

pub fn random_swap(original_solution: &Solution) -> Solution {
    let mut solution_mutant = original_solution.clone();
    let len = original_solution.dimension;
    if len > 1 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..original_solution.dimension);
        let j = (rng.gen_range(0..original_solution.dimension-1) + i + 1) % original_solution.dimension;
        solution_mutant.order.swap(i, j);
    }
    solution_mutant
}

pub fn measure_execution_time<F: Fn()>(f: F) -> f64{
    let mut total_duration = Duration::new(0, 0);
    let mut iterations = 0;
    let start = Instant::now();

    while total_duration.as_nanos() < 10 || iterations < 10{
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
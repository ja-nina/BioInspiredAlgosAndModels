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
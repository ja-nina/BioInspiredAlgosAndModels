mod atsp;
mod solution;
mod errors;
mod utils;

use atsp::ATSP;
use solution::Solution;
use utils::randomize_by_swaps;
use utils::measure_execution_time;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let atsp = ATSP::read_from_file("../data/ALL_atsp/br17.atsp")?;
    atsp.display();

    let cities: Vec<i32> = (0..17).collect();
    let initial_sol = Solution::new(&cities)?;

    atsp.is_solution_valid(&initial_sol)?;
    let cost_initial = atsp.cost_of_solution(&initial_sol);
    println!("Initial solution cost {}", cost_initial);

    let randomized_solution = randomize_by_swaps(&initial_sol);
    let cost_randomized = atsp.cost_of_solution(&randomized_solution);
    println!("Randomized solution cost : {}", cost_randomized);


    let avg_time = measure_execution_time(|| {
        randomize_by_swaps(&initial_sol);
    });

    println!("Average time for randomization: {} nanoseconds", avg_time);

    Ok(())
}
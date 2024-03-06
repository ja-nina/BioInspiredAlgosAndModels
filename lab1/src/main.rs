mod atsp;
mod solution;
mod errors;
mod utils;

use atsp::ATSP;
use solution::Solution;
use utils::randomize_by_swaps;
use utils::measure_execution_time;
use utils::random_swap;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let atsp = ATSP::read_from_file("../data/ALL_atsp/br17.atsp")?;
    atsp.display();

    let cities: Vec<i32> = (0..17).collect();
    let initial_sol = Solution::new(&cities)?;

    let mutated_sol = random_swap(&initial_sol);
    let nn_solution = atsp.nearest_neigbor();

    println!("Initial solution order: {:?}", initial_sol.order);
    println!("Swap Mutation solution order: {:?}", mutated_sol.order);
    println!("Nearest neighbor solution order: {:?}", nn_solution.order);

    atsp.is_solution_valid(&initial_sol)?;
    let cost_initial = atsp.cost_of_solution(&initial_sol);

    let randomized_solution = randomize_by_swaps(&initial_sol);
    let cost_randomized = atsp.cost_of_solution(&randomized_solution);
    
    println!("Initial solution cost {}", cost_initial);
    println!("Randomized solution cost : {}", cost_randomized);
    println!("Nearest neighbor solution cost : {}", atsp.cost_of_solution(&nn_solution));


    let avg_time = measure_execution_time(|| {
        randomize_by_swaps(&initial_sol);
    });

    println!("Average time for randomization: {} nanoseconds", avg_time);

    let avg_time = measure_execution_time(|| {
        random_swap(&initial_sol);
    });

    println!("Average time for single swap: {} nanoseconds", avg_time);

    Ok(())
}
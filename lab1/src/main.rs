mod atsp;
mod solution;
mod errors;
mod args;
mod utils;
mod operation;

use clap::Parser;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Opt::parse();
    let atsp = atsp::ATSP::read_from_file(&args.instance)?;

    // TODO: use rng in random functions
    let mut rng = StdRng::seed_from_u64(args.seed);
    
    if args.verbose {
        atsp.display();
    }

    let cities: Vec<i32> = (0..17).collect();
    let initial_sol = solution::Solution::new(&cities)?;

    let mutated_sol = utils::random_swap(&initial_sol, &mut rng);
    let nn_solution = atsp.nearest_neigbor();

    if args.verbose {
        println!("Initial solution order: {:?}", initial_sol.order);
        println!("Swap Mutation solution order: {:?}", mutated_sol.order);
        println!("Nearest neighbor solution order: {:?}", nn_solution.order);
    }

    atsp.is_solution_valid(&initial_sol)?;
    let cost_initial = atsp.cost_of_solution(&initial_sol);

    let randomized_solution = utils::randomize_by_swaps(&initial_sol, &mut rng);
    let cost_randomized = atsp.cost_of_solution(&randomized_solution);
    
    if args.verbose {
        println!("Initial solution cost {}", cost_initial);
        println!("Randomized solution cost : {}", cost_randomized);
        println!("Nearest neighbor solution cost : {}", atsp.cost_of_solution(&nn_solution));
    }

    let avg_time_randomize = utils::measure_execution_time(|| {
        utils::randomize_by_swaps(&initial_sol, &mut rng);
    });
    let avg_time_swap = utils::measure_execution_time(|| {
        utils::random_swap(&initial_sol, &mut rng);
    });

    if args.verbose {
        println!("Average time for randomization: {}", utils::humanize_time(avg_time_randomize));
        println!("Average time for single swap: {}", utils::humanize_time(avg_time_swap));
    }

    Ok(())
}
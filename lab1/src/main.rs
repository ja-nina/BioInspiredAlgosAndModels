mod args;
mod atsp;
mod errors;
mod explorers;
mod initializers;
mod operation;
mod search;
mod solution;
mod utils;

use clap::Parser;

// TODO: Implement RandomWalk
// TODO: Implement Heuristic Search
// TODO: Implement Greedy Local Search
// TODO: Implement Steepest Local Search

fn explorer_from_args(args: &args::Opt) -> Box<dyn search::Explorer> {
    match args.algorithm {
        args::Algorithms::Random => Box::new(explorers::RandomExplorer::new(
            args.seed,
            args.max_iterations,
        )),
    }
}

fn initializer_from_args(args: &args::Opt) -> Box<dyn search::Initializer> {
    Box::new(initializers::RandomInitializer::new(args.seed))
}

fn solution_from_args(
    args: &args::Opt,
    instance: &atsp::ATSP,
) -> (solution::Solution, search::Context) {
    let mut explorer: Box<dyn search::Explorer> = explorer_from_args(&args);
    let mut initializer: Box<dyn search::Initializer> = initializer_from_args(&args);
    let mut search_alg = search::SearchAlgorithm::new(instance, &mut initializer, &mut explorer);
    search_alg.run()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Opt::parse();
    let atsp = atsp::ATSP::read_from_file(&args.instance)?;

    if args.verbose {
        atsp.display(false);
    }

    let (solution, ctx) = solution_from_args(&args, &atsp);

    atsp.is_solution_valid(&solution)?;
    if args.verbose {
        println!("\n========= DONE ==========");
        println!("{:#?}", ctx);
        println!("Solution Cost: {}", atsp.cost_of_solution(&solution));
    }

    if !args.time {
        return Ok(());
    }
    let avg_running_time = utils::measure_execution_time(|| {
        solution_from_args(&args, &atsp);
    });
    if args.verbose {
        println!("Time taken: {}", utils::humanize_time(avg_running_time));
    };

    // TODO: Implement Exporting the results to a file

    Ok(())
}

mod args;
mod atsp;
mod deltas;
mod errors;
mod explorers;
mod export;
mod initializers;
mod operation;
mod search;
mod solution;
mod utils;

use args::alg_as_str;
use clap::Parser;

// TODO: Implement 3-opt operation

fn explorer_from_args(args: &args::Opt, instance: &atsp::ATSP) -> Box<dyn search::Explorer> {
    match args.algorithm {
        args::Algorithm::Random => {
            Box::new(explorers::RandomExplorer::new(args.seed, args.max_time_ns))
        }
        args::Algorithm::RandomWalk => Box::new(explorers::RandomWalkExplorer::new(
            args.seed,
            args.max_time_ns,
        )),
        args::Algorithm::GreedySearch => Box::new(explorers::GreedySearchExplorer::new(
            args.seed,
            instance.dimension as u16,
        )),
        args::Algorithm::SteepestSearch => {
            Box::new(explorers::SteepestSearchExplorer::new(args.seed))
        }
        args::Algorithm::NNHeuristic => Box::new(explorers::PassThroughExplorer {}),
    }
}

fn initializer_from_args(args: &args::Opt) -> Box<dyn search::Initializer> {
    match args.algorithm {
        args::Algorithm::NNHeuristic => Box::new(initializers::NearestNeighborInitializer {}),
        _ => Box::new(initializers::RandomInitializer::new(args.seed)),
    }
}

fn solution_from_args(
    args: &args::Opt,
    instance: &atsp::ATSP,
) -> (solution::Solution, search::Context) {
    let mut explorer: Box<dyn search::Explorer> = explorer_from_args(&args, &instance);
    let mut initializer: Box<dyn search::Initializer> = initializer_from_args(&args);
    let mut search_alg = search::SearchAlgorithm::new(instance, &mut initializer, &mut explorer);
    search_alg.run()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Opt::parse();
    let atsp = atsp::ATSP::read_from_file(&args.instance)?;

    assert!(atsp.dimension == atsp.matrix.len() && atsp.dimension > 0);

    if args.verbose {
        atsp.display(false);

        println!("\n========= CONFIG ==========");
        println!("{:#?}", args);
    }

    let (solution, ctx) = solution_from_args(&args, &atsp);

    atsp.is_solution_valid(&solution)?;
    let score = atsp.cost_of_solution(&solution);
    if args.verbose {
        println!("\n========= DONE ==========");
        println!("{:#?}", ctx);
        println!("Solution Cost: {}", score);

        let it = operation::NeighborhoodIterator::new(atsp.dimension as u16);
        println!("Neighborhood Size: {}", it.size());
    }

    let mut avg_running_time: f64 = -1.0;
    if args.time {
        avg_running_time = utils::measure_execution_time(|| {
            solution_from_args(&args, &atsp);
        });
        if args.verbose {
            println!("Time taken: {}", utils::humanize_time(avg_running_time));
        };
    }

    export::export_to_file(
        &args.output,
        &solution,
        score,
        avg_running_time,
        ctx.iterations,
        ctx.steps,
        ctx.evaluations,
        alg_as_str(&args.algorithm),
        atsp.name.as_str(),
    );

    Ok(())
}

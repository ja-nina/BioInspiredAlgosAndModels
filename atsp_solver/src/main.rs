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

fn op_flags_from_args(args: &args::Opt) -> u32 {
    let mut op_flags = operation::OperationFlags::NODE_SWAP | operation::OperationFlags::EDGE_SWAP;
    if args.edge_swap == 0 {
        op_flags.remove(operation::OperationFlags::EDGE_SWAP);
    }
    if args.node_swap == 0 {
        op_flags.remove(operation::OperationFlags::NODE_SWAP);
    }
    op_flags.bits()
}

fn explorer_from_args(args: &args::Opt, instance: &atsp::ATSP) -> Box<dyn search::Explorer> {
    let op_flags = op_flags_from_args(&args);
    match args.algorithm {
        args::Algorithm::Random => {
            Box::new(explorers::RandomExplorer::new(args.seed, args.max_time_ns))
        }
        args::Algorithm::RandomWalk => Box::new(explorers::RandomWalkExplorer::new(
            args.seed,
            args.max_time_ns,
            op_flags,
        )),
        args::Algorithm::GreedySearch | args::Algorithm::GreedySearchNN => Box::new(
            explorers::GreedySearchExplorer::new(args.seed, instance.dimension as u16, op_flags),
        ),
        args::Algorithm::SteepestSearchNN | args::Algorithm::SteepestSearch => {
            Box::new(explorers::SteepestSearchExplorer::new(args.seed, op_flags))
        }
        args::Algorithm::NNHeuristic => Box::new(explorers::PassThroughExplorer {}),
        args::Algorithm::SimulatedAnnealing | args::Algorithm::SimulatedAnnealingNN => {
            Box::new(explorers::SimulatedAnnealingExplorer::new(
                args.seed,
                op_flags,
                args.meta_param_1,
                args.meta_param_2,
                args.meta_param_3 as u32,
            ))
        }
        args::Algorithm::TabuSearch | args::Algorithm::TabuSearchNN => {
            Box::new(explorers::TabuSearchExplorer::new(
                op_flags,
                args.meta_param_2 as u32,
                args.meta_param_3 as u32,
            ))
        }
    }
}

fn initializer_from_args(args: &args::Opt) -> Box<dyn search::Initializer> {
    match args.algorithm {
        args::Algorithm::NNHeuristic
        | args::Algorithm::GreedySearchNN
        | args::Algorithm::SteepestSearchNN
        | args::Algorithm::SimulatedAnnealingNN
        | args::Algorithm::TabuSearchNN => {
            Box::new(initializers::NearestNeighborInitializer::new(args.seed))
        }
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

        let it =
            operation::NeighborhoodIterator::new(atsp.dimension as u16, op_flags_from_args(&args));
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
    let mut neigborhood_type = "both";
    if args.edge_swap == 0 {
        neigborhood_type = "node";
    } else if args.node_swap == 0 {
        neigborhood_type = "edge";
    }

    if args.output == "" {
        return Ok(());
    }
    export::export_to_file(
        &args.output,
        &solution,
        ctx.initial_cost,
        score,
        avg_running_time,
        ctx.iterations,
        ctx.steps,
        ctx.evaluations,
        alg_as_str(&args.algorithm),
        atsp.name.as_str(),
        neigborhood_type,
    );

    Ok(())
}

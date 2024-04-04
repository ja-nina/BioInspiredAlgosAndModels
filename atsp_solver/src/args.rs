use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Algorithm {
    Random,
    RandomWalk,
    NNHeuristic,
    GreedySearch,
    GreedySearchNN,
    SteepestSearch,
    SteepestSearchNN,
    SimulatedAnnealing,
    SimulatedAnnealingNN,
    TabuSearch,
    TabuSearchNN,
}

pub fn alg_as_str(alg: &Algorithm) -> &str {
    match alg {
        Algorithm::Random => "random",
        Algorithm::RandomWalk => "random-walk",
        Algorithm::NNHeuristic => "nn-heuristic",
        Algorithm::GreedySearch => "greedy-search",
        Algorithm::SteepestSearch => "steepest-search",
        Algorithm::GreedySearchNN => "greedy-search-nn",
        Algorithm::SteepestSearchNN => "steepest-search-nn",
        Algorithm::SimulatedAnnealing => "simulated-annealing",
        Algorithm::SimulatedAnnealingNN => "simulated-annealing-nn",
        Algorithm::TabuSearch => "tabu-search",
        Algorithm::TabuSearchNN => "tabu-search-nn",
    }
}

#[derive(Parser, Debug)]
#[command(name = "ATSP", about = "Solve ATSP problems")]
pub struct Opt {
    /// Path to the file with the ATSP problem
    #[arg(short, long)]
    pub instance: String,

    /// Seed for the random number generator
    #[arg(short, long, default_value = "0")]
    pub seed: u64,

    /// Print output in a verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Maximum running time in nanoseconds
    /// Only used for iterative algorithms
    #[arg(short, long, default_value = "0")]
    pub max_time_ns: u64,

    /// Algorithm to use
    #[arg(short, long, value_enum)]
    pub algorithm: Algorithm,

    /// Measure the time of execution
    #[arg(short, long)]
    pub time: bool,

    /// Output file path
    #[arg(short, long, default_value = "")]
    pub output: String,

    /// Use edge swap neighborhood
    /// Only used for iterative algorithms
    #[arg(long, default_value = "1")]
    pub edge_swap: u32,

    /// Use node swap neighborhood
    /// Only used for iterative algorithms
    #[arg(long, default_value = "1")]
    pub node_swap: u32,

    /// Meta parameter 1 for algorithms
    /// For Simulated Annealing, it is the cooling rate
    /// For Tabu Search, it is the tabu tenure multiplier
    #[arg(long, default_value = "0.5")]
    pub meta_param_1: f64,

    /// Meta parameter 2 for algorithms
    /// For Simulated Annealing, it is the initial temperature
    /// For Tabu Search, it is the aspiration criterion
    #[arg(long, default_value = "100.0")]
    pub meta_param_2: f64,

    /// Meta parameter 3 for algorithms
    /// For Simulated Annealing, it is the markov chain length multiplier
    /// For Tabu Search, it is the candidate list size multiplier
    #[arg(long, default_value = "1.0")]
    pub meta_param_3: f64,
}

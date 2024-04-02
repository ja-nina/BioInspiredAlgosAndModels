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

    /// Cooling rate
    /// Only used for Simulated Annealing
    /// Must be in the range (0, 1)
    /// The closer to 1, the slower the decay
    #[arg(long, default_value = "0.999")]
    pub cooling_rate: f64,

    /// Initial temperature
    /// Only used for Simulated Annealing
    /// Must be greater than 0
    /// The higher the temperature, the more likely to accept worse solutions
    #[arg(long, default_value = "100.0")]
    pub initial_temperature: f64,
}

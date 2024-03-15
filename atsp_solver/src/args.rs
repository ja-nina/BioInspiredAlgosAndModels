use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Algorithm {
    Random,
    RandomWalk,
    GreedySearch,
    SteepestSearch,
}

pub fn alg_as_str(alg: &Algorithm) -> &str {
    match alg {
        Algorithm::Random => "random",
        Algorithm::RandomWalk => "random-walk",
        Algorithm::GreedySearch => "greedy-search",
        Algorithm::SteepestSearch => "steepest-search",
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

    // TODO: replace max_iterations with max running time for R and RW
    /// Maximum number of iterations
    /// Only used for iterative algorithms
    #[arg(short, long, default_value = "1000")]
    pub max_iterations: u32,

    /// Algorithm to use
    #[arg(short, long, value_enum)]
    pub algorithm: Algorithm,

    /// Measure the time of execution
    #[arg(short, long)]
    pub time: bool,

    /// Output file path
    #[arg(short, long)]
    pub output: String,
}

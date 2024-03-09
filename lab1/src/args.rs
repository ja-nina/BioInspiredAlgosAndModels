use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Algorithms {
    Random,
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

    /// Maximum number of iterations
    /// Only used for iterative algorithms
    #[arg(short, long, default_value = "1000")]
    pub max_iterations: u32,

    /// Algorithm to use
    /// r: Random
    #[arg(short, long, value_enum)]
    pub algorithm: Algorithms,

    /// Measure the time of execution
    #[arg(short, long)]
    pub time: bool,
}

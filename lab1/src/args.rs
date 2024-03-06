use clap::Parser;

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
}

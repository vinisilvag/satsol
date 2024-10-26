use clap::Parser;

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Solver {
    Brute,
    Dpll,
    Cdcl,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Solving strategy
    #[arg(short, long)]
    pub solver: Solver,

    /// The input file in DIMACS format
    #[arg(short, long)]
    pub input_file: String,
}

mod args;
mod parser;
mod solvers;

use args::{Args, Solver};
use clap::Parser as ClapParser;
use parser::Parser;
use solvers::brute;

fn main() {
    let Args { solver, input_file } = Args::parse();
    let parser = Parser::new(input_file);
    let problem = parser.parse().unwrap();

    match solver {
        Solver::Brute => brute::solve_brute(problem),
        _ => {
            todo!("{:?} solver not implemented yet", solver)
        }
    }
}

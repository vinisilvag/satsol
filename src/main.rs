mod args;
mod parser;
mod solvers;

use args::{Args, Solver};
use clap::Parser as ClapParser;
use parser::Parser as InputParser;
use solvers::brute;

fn main() {
    let Args { solver, input_file } = Args::parse();
    let parser = InputParser::new(input_file);
    let problem = parser.parse().unwrap();

    let ans = match solver {
        Solver::Brute => brute::solve_brute(&problem),
        _ => {
            todo!("{:?} solver not implemented", solver)
        }
    };

    dbg!(&ans);

    match ans {
        Some(_) => println!("sat"),
        None => println!("unsat"),
    }
}

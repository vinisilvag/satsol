mod args;
mod parser;
mod solvers;

use args::{Args, Solver};
use clap::Parser as ClapParser;
use parser::Parser as InputParser;
use solvers::{brute, dpll, Model, Satisfiability};
use std::time::{Duration, Instant};

fn show_assignment(model: Model) {
    for (i, assignment) in model.iter().enumerate() {
        if let Some(val) = assignment {
            println!("x_{:?} = {:?}", i + 1, val);
        }
    }
}

fn main() {
    let Args { solver, input_file } = Args::parse();
    let mut parser = InputParser::new(input_file);
    let problem = match parser.parse() {
        Ok(problem) => problem,
        Err(error) => panic!("{error}"),
    };

    let start: Instant = Instant::now();
    let solution = match solver {
        Solver::Brute => brute::solve_brute(&problem),
        Solver::Dpll => dpll::solve_dpll(&problem),
        _ => {
            todo!("{:?} solver", solver)
        }
    };
    let duration: Duration = start.elapsed();

    match solution {
        Satisfiability::Sat(model) => {
            println!("model:");
            show_assignment(model);
            println!("\nsat");
        }
        Satisfiability::Unsat => {
            println!("unsat")
        }
    }
    println!("time elapsed: {:?}", duration);
}

mod args;
mod parser;
mod solvers;

use args::{Args, Solver};
use clap::Parser as ClapParser;
use parser::Parser as InputParser;
use solvers::{brute, Model, Satisfiability};

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
    let problem = parser.parse().unwrap();

    let solution = match solver {
        Solver::Brute => brute::solve_brute(&problem),
        _ => {
            todo!("{:?} solver not implemented", solver)
        }
    };

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
}

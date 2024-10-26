use crate::{
    parser::Problem,
    solvers::{Model, Satisfiability},
};

pub fn solve_dpll(problem: &Problem) -> Satisfiability {
    fn solve_rec(problem: &Problem, model: Model) -> Option<Model> {
        dbg!(problem);
        dbg!(model);
        None
    }

    let model: Model = vec![None; problem.variables_num];
    match solve_rec(problem, model) {
        Some(model) => Satisfiability::Sat(model),
        None => Satisfiability::Unsat,
    }
}

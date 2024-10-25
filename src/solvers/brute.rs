use crate::{
    parser::Problem,
    solvers::{assign_truth_value, check_sat, check_unsat_clause, Satisfiability},
};

use super::Model;

pub fn solve_brute(problem: &Problem) -> Satisfiability {
    fn solve_rec(problem: &Problem, model: Model) -> Option<Model> {
        if check_sat(&problem.formula, &model) {
            return Some(model);
        }
        if check_unsat_clause(&problem.formula, &model) {
            return None;
        }

        let choice = model.iter().position(|x| x.is_none())?;

        let left = solve_rec(
            problem,
            assign_truth_value(model.clone(), choice as usize, false),
        );
        if left.is_some() {
            left
        } else {
            solve_rec(
                problem,
                assign_truth_value(model.clone(), choice as usize, true),
            )
        }
    }

    let model = vec![None; problem.variables_num];
    match solve_rec(problem, model) {
        Some(model) => Satisfiability::Sat(model),
        None => Satisfiability::Unsat,
    }
}

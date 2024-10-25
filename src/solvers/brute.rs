use crate::parser::{Formula, Problem};

use super::Model;

fn assign(model: Model, i: usize, val: bool) -> Model {
    let mut new_model = model.clone();
    new_model[i] = Some(val);
    new_model
}

fn check_sat(formula: &Formula, model: &Model) -> bool {
    for clause in formula {
        let clause_is_satisfied = clause
            .iter()
            .filter(|&&x| {
                if let Some(val) = model[x.abs() as usize - 1] {
                    if x > 0 {
                        val
                    } else {
                        !val
                    }
                } else {
                    false
                }
            })
            .count()
            != 0;
        if !clause_is_satisfied {
            return false;
        }
    }
    true
}

fn check_unsat_clause(formula: &Formula, model: &Model) -> bool {
    for clause in formula {
        let clause_is_unsat = clause
            .iter()
            .filter(|&&x| {
                if let Some(val) = model[x.abs() as usize - 1] {
                    if x > 0 {
                        val
                    } else {
                        !val
                    }
                } else {
                    true
                }
            })
            .count()
            == 0;
        if clause_is_unsat {
            return true;
        }
    }
    false
}

pub fn solve_brute(problem: &Problem) -> Option<Model> {
    fn solve_rec(problem: &Problem, model: Model) -> Option<Model> {
        if check_sat(&problem.formula, &model) {
            return Some(model);
        }
        if check_unsat_clause(&problem.formula, &model) {
            return None;
        }

        let choice = model.iter().position(|x| x.is_none())?;

        let left = solve_rec(problem, assign(model.clone(), choice as usize, false));
        if left.is_some() {
            left
        } else {
            solve_rec(problem, assign(model.clone(), choice as usize, true))
        }
    }

    let model = vec![None; problem.variables_num];
    solve_rec(problem, model)
}

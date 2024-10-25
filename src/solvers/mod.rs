use crate::parser::{Formula, Variable};

pub mod brute;

pub type Model = Vec<Option<bool>>;

#[derive(Debug)]
pub enum Satisfiability {
    Sat(Model),
    Unsat,
}

pub fn assign_truth_value(model: Model, i: usize, val: bool) -> Model {
    let mut new_model = model.clone();
    new_model[i] = Some(val);
    new_model
}

pub fn check_sat(formula: &Formula, model: &Model) -> bool {
    for clause in formula {
        let clause_is_satisfied = clause
            .iter()
            .filter(|&x| match x {
                Variable::Literal(lit) => model[lit - 1].unwrap_or_else(|| false),
                Variable::NegLiteral(lit) => {
                    if let Some(val) = model[lit - 1] {
                        !val
                    } else {
                        false
                    }
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

pub fn check_unsat_clause(formula: &Formula, model: &Model) -> bool {
    for clause in formula {
        let clause_is_unsat = clause
            .iter()
            .filter(|&x| match x {
                Variable::Literal(lit) => model[lit - 1].unwrap_or_else(|| true),
                Variable::NegLiteral(lit) => {
                    if let Some(val) = model[lit - 1] {
                        !val
                    } else {
                        true
                    }
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

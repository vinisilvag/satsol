use error::SolverError;

use crate::parser::{Formula, Variable};

pub mod brute;
pub mod dpll;
pub mod error;

pub type Model = Vec<Option<bool>>;

#[derive(Debug)]
pub enum Satisfiability {
    Sat(Model),
    Unsat,
}

pub fn assign_truth_value(model: Model, i: usize, val: bool) -> Result<Model, SolverError> {
    if i >= model.len() {
        return Err(SolverError::VariableOutOfBounds(i, model.len()));
    }
    let mut new_model = model.clone();
    new_model[i] = Some(val);
    Ok(new_model)
}

pub fn check_sat(formula: &Formula, model: &Model) -> bool {
    for clause in formula {
        let clause_is_satisfied = clause
            .iter()
            .filter(|&x| match x {
                Variable::Literal(lit) => model[lit.to_owned()].unwrap_or(false),
                Variable::NegLiteral(lit) => {
                    if let Some(val) = model[lit.to_owned()] {
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
                Variable::Literal(lit) => model[lit.to_owned()].unwrap_or(true),
                Variable::NegLiteral(lit) => {
                    if let Some(val) = model[lit.to_owned()] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_truth_value() {
        let m_0: Model = vec![None; 4];

        let m_1 = assign_truth_value(m_0, 0, true).unwrap();
        assert!(m_1[0].unwrap());

        let m_2 = assign_truth_value(m_1, 2, false).unwrap();
        assert!(m_2[0].unwrap());
        assert!(!m_2[2].unwrap());

        let m_3 = assign_truth_value(m_2, 0, false).unwrap();
        assert!(!m_3[0].unwrap());

        let m_4 = assign_truth_value(m_3, 4, true);
        assert!(m_4.is_err());
    }

    #[test]
    fn test_check_sat() {}

    #[test]
    fn test_check_unsat_clause() {}
}

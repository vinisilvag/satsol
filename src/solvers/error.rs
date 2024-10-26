use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("trying to assign a truth value to x_`{0}` but the last variable is x_`{1}` (out of bounds)")]
    VariableOutOfBounds(usize, usize),
}

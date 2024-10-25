//! A parser for the DIMACS .cnf format.

pub mod error;
pub mod lexer;

use std::fs::File;
use std::io::{self};

use error::ParserError;
use lexer::{Lexer, Line};

#[derive(Clone, Debug)]
pub enum Variable {
    Literal(usize),
    NegLiteral(usize),
}

type Clause = Vec<Variable>;
pub type Formula = Vec<Clause>;

#[derive(Debug)]
pub struct Problem {
    pub variables_num: usize,
    pub clauses_num: usize,
    pub formula: Formula,
}

pub struct Parser {
    input_file: String,
    lexer: Lexer,
}

impl Parser {
    pub fn new(input_file: String) -> Self {
        let file = File::open(&input_file).unwrap();
        let reader = io::BufReader::new(file);
        let lexer = Lexer::new(reader);
        Parser { input_file, lexer }
    }

    pub fn parse(&mut self) -> Result<Problem, ParserError> {
        let mut header_readed = false;
        let mut variables_num = 0;
        let mut clauses_num = 0;
        let mut formula: Formula = Vec::new();

        loop {
            let line = self.lexer.read_line()?;
            match line {
                Line::Header(_, variables, clauses) => {
                    variables_num = variables;
                    clauses_num = clauses;
                    header_readed = true;
                }
                Line::Clause(variables) => {
                    if !header_readed {
                        return Err(ParserError::ClauseBeforeHeader);
                    }

                    let formatted: Clause = variables
                        .iter()
                        .map(|&x| {
                            if x > 0 {
                                Variable::Literal(x as usize)
                            } else {
                                Variable::NegLiteral((-x) as usize)
                            }
                        })
                        .collect();
                    formula.push(formatted);
                }
                Line::End => {
                    break;
                }
            }
        }

        Ok(Problem {
            variables_num,
            clauses_num,
            formula,
        })
    }
}

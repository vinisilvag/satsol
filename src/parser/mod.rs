//! A parser for the DIMACS .cnf format.

use std::fs::File;
use std::io::{self, BufRead};

pub struct Parser {
    input_file: String,
}

type Clause = Vec<i32>;
pub type Formula = Vec<Clause>;

#[derive(Debug)]
pub struct Problem {
    pub variables_num: usize,
    pub clauses_num: usize,
    pub formula: Formula,
}

impl Parser {
    pub fn new(input_file: String) -> Self {
        Parser { input_file }
    }

    pub fn parse(self) -> Result<Problem, io::Error> {
        let file = File::open(self.input_file)?;
        let reader = io::BufReader::new(file);

        let mut variables_num: usize = 0;
        let mut clauses_num: usize = 0;
        let mut formula: Formula = Vec::new();
        let mut curr_clause = 0;

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.starts_with("0") {
                break;
            }
            if trimmed.starts_with("c") || trimmed.starts_with("%") {
                continue;
            }
            if trimmed.starts_with("p") {
                let parts: Vec<&str> = trimmed.split(" ").collect();
                let filtered: Vec<&&str> = parts.iter().filter(|x| **x != "").collect();
                if *filtered[1] != "cnf" {
                    panic!("input formula not in cnf")
                }
                variables_num = filtered[2].parse::<usize>().unwrap();
                clauses_num = filtered[3].parse::<usize>().unwrap();
                formula.resize(clauses_num, vec![]);
                continue;
            }

            let parts: Vec<&str> = trimmed.split(" ").collect();
            let filtered: Vec<&&str> = parts.iter().filter(|x| **x != "").collect();
            for lit in filtered {
                let lit = lit.parse::<i32>().unwrap();
                // clause can be written in multiple lines
                if lit == 0 {
                    curr_clause += 1;
                    break;
                }
                formula[curr_clause].push(lit);
            }
        }

        Ok(Problem {
            variables_num,
            clauses_num,
            formula,
        })
    }
}

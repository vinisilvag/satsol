//! A lexer for the DIMACS .cnf format.

use std::fs::File;
use std::io::{BufRead, BufReader};

use super::error::ParserError;

#[derive(Debug)]
pub enum Line {
    Header(String, usize, usize),
    Clause(Vec<i32>),
    End,
}

pub struct Lexer {
    reader: BufReader<File>,
}

impl Lexer {
    pub fn new(reader: BufReader<File>) -> Self {
        Lexer { reader }
    }

    pub fn read_line(&mut self) -> Result<Line, ParserError> {
        loop {
            let mut buf = String::new();
            self.reader.read_line(&mut buf).unwrap();

            if buf != "" {
                let trimmed = buf.trim();
                if trimmed.starts_with("c") || trimmed.starts_with("%") {
                    continue;
                }
                if trimmed.starts_with("0") {
                    return Ok(Line::End);
                }

                if trimmed.starts_with("p") {
                    let parts: Vec<&str> = trimmed.split(" ").collect();
                    let filtered: Vec<&&str> = parts.iter().filter(|&&x| x != "").collect();
                    let [typ, format, variables, clauses] = filtered[..] else {
                        return Err(ParserError::HeaderPoorlyFormatted);
                    };
                    if format.to_owned() != "cnf" {
                        return Err(ParserError::InputFormatNotSupported(format.to_string()));
                    }
                    return Ok(Line::Header(
                        typ.to_string(),
                        variables.parse::<usize>().unwrap(),
                        clauses.parse::<usize>().unwrap(),
                    ));
                } else {
                    let parts: Vec<&str> = trimmed.split(" ").collect();
                    let filtered: Vec<&&str> = parts.iter().filter(|&&x| x != "").collect();
                    let parsed: Vec<i32> = filtered
                        .iter()
                        .map(|x| x.parse::<i32>().unwrap())
                        .take(filtered.len() - 1)
                        .collect();
                    return Ok(Line::Clause(parsed));
                }
            } else {
                return Ok(Line::End);
            }
        }
    }
}

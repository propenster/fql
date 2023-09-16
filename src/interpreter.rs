use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use thiserror::Error;

use crate::{ast::*, token::Token};
use regex::Regex;

const DEFAULT_N_COUNT: usize = 10;
pub fn interprete(program: Program) -> Result<(), Box<dyn std::error::Error>> {
    let mut ast = program.iter();

    while let Some(statement) = ast.next() {
        execute_statement(statement)?;
    }

    Ok(())
}

fn execute_statement(statement: &Statement) -> Result<(), InterpreterError> {
    let mut query = String::new();

    match statement {
        Statement::SelectStatement {
            counters,
            quantity,
            from,
            target,
            condition,
        } => {
            let mut path = Path::new("");
            println!("Matched SelectStatement variant:");
            if let Some(qty) = quantity {
                println!("Quantity: {:?}", qty);
            } else {
                println!("Quantity: None");
            }
            println!("From: {:?}", from);
            println!("Target: {:?}", target);
            if target.is_empty() {
                return Err(InterpreterError::InvalidOrUnavailableFile(target.clone()));
            }
            path = Path::new(target);
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                let mut lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

                //DO WHERE conditionals...here
                if let Some(cond) = condition {
                    println!("Condition: {:?}", cond);
                    // let lines_containing_phrase: Vec<String> = my_vec
                    //     .iter()
                    //     .filter(|line| line.contains(phrase_to_find))
                    //     .cloned()
                    //     .collect();
                    match cond {
                        Expression::Where {
                            conditional,
                            target,
                        } => match conditional {
                            Some(Token::Like) => {
                                lines = lines
                                    .iter()
                                    .filter(|line| line.contains(target))
                                    .cloned()
                                    .collect();
                            }
                            Some(Token::NotLike) => {
                                lines = lines
                                    .iter()
                                    .filter(|line| !line.contains(target))
                                    .cloned()
                                    .collect();
                            }
                            _ => unreachable!(),
                        },

                        _ => {
                            return Err(InterpreterError::InvalidQueryToken(
                                "* or TOP or TAIL token after SELECT".to_owned(),
                            ))
                        }
                    }
                } else {
                    println!("Condition: None");
                }

                if let Some(qty) = quantity {
                    match qty {
                        Token::Star => {
                            if let Some(counter) = counters {
                                run_counter(counter, &lines);
                            } else {
                                for line in lines {
                                    println!("{}", line);
                                }
                            }
                        }
                        Token::Top(s) => {
                            //println!("Top({})", s);
                            let n = get_n_from_token(s);
                            let top_n_lines = &lines[..n];
                            if let Some(counter) = counters {
                                run_counter(counter, &top_n_lines);
                            } else {
                                for line in top_n_lines.iter() {
                                    //println!("{}: {}", i + 1, line);
                                    println!("{}", line);
                                }
                            }
                        }
                        Token::Tail(s) => {
                            //println!("Tail({})", s);
                            let n = get_n_from_token(s);
                            let total_lines = lines.len();
                            let bottom_n_lines = &lines[total_lines - n..];

                            if let Some(counter) = counters {
                                run_counter(counter, &bottom_n_lines);
                            } else {
                                for line in bottom_n_lines.iter() {
                                    //println!("{}: {}", i + 1, line);
                                    println!("{}", line);
                                }
                            }
                        }

                        _ => unreachable!(),
                    }
                } else {
                    println!("Invalid query token... ");
                    return Err(InterpreterError::InvalidQueryToken(
                        "* or TOP or TAIL token after SELECT".to_owned(),
                    ));
                }
            } else {
                return Err(InterpreterError::InvalidOrUnavailableFile(target.clone()));
            }

            //do the actual running here...
            Ok(())
        }
        _ => todo!(),
    }
}

fn run_counter(counter: &Token, lines: &[String]) {
    match counter {
        Token::CountC => {
            let total_chars = lines.iter().map(|s| s.chars().count()).sum::<usize>();

            println!("{}", total_chars);
        }
        Token::CountL => {
            println!("{}", lines.len());
        }
        Token::CountW => {
            let total_words = lines
                .iter()
                .map(|s| s.split_whitespace().count())
                .sum::<usize>();

            println!("{}", total_words);
        }

        _ => unreachable!(),
    }
}

fn get_n_from_token(token: &String) -> usize {
    let re = Regex::new(r"\d+(\.\d+)?").unwrap();
    let mut total: usize = 0;
    // Find all matches in the text
    let matches: Vec<&str> = re.find_iter(token.as_str()).map(|m| m.as_str()).collect();

    // Print the matched numbers
    for num in matches {
        if let Ok(n) = num.parse::<usize>() {
            total += n;
        } else {
            total += DEFAULT_N_COUNT
        }
    }

    total
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Invalid or Unavailable. Something is preventing read of this file '{0}' ")]
    InvalidOrUnavailableFile(String),

    #[error("Invalid Query Token. Require '{0}'")]
    InvalidQueryToken(String),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

// impl From<std::io::Error> for InterpreterError {
//     fn from(error: std::io::Error) -> Self {
//         InterpreterError::InvalidOrUnavailableFile(error.to_string())
//         //InterpreterError::InvalidQueryToken(error.to_string())
//     }
// }

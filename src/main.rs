use std::{fs, env::args};


mod token;
mod parser;
mod interpreter;
mod ast;

fn main() {
    println!("Hello, world!");
    let file = args().nth(1).unwrap();
    let contents = fs::read_to_string(file).unwrap();

    let tokens = token::generate(contents.as_str().trim());

    dbg!(&tokens);

    match parser::parse(tokens) {
        Ok(ast) => {
            match interpreter::interprete(ast) {
                Ok(_) => {
                    
                },
                Err(e) => eprintln!("Error occurred while parsing token >>> {}", e) ,//e.print(),
            };
        },
        Err(e) => eprintln!("Error occurred while parsing token >>> {}", e) , // e.print(),
    };

}

extern crate regex;
mod parser;
mod mir;
use mir::Mir;
use parser::ast::ParserState;
use parser::lexer::Tokenizer;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let mut f = File::open(&args[1]).unwrap();
        let mut program = String::new();
        if let Err(e) = f.read_to_string(&mut program) {
            println!("Error reading file: {:?}", e);
            return;
        }
        let tokens = match Tokenizer::tokenize(&program) {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };
        println!("Tokens: {:?}\n", tokens);

        let program = match ParserState::parse(tokens) {
            Ok(program) => program,
            Err(e) => {
                println!("Parser error: {:?}", e);
                return;
            }
        };
        println!("Program: {:?}\n", program);

        let mir = match Mir::from_program(program) {
            Ok(mir) => mir,
            Err(e) => {
                println!("MIR parsing error: {:?}", e);
                return;
            }
        };
        println!("MIR: {:?}\n", mir);
    }
}

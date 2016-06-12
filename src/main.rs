extern crate regex;
mod parser;
use parser::lexer::Tokenizer;
use parser::ast::ParserState;

fn main() {
    let program = "mov P3, #0AAh\nret\n";
    let tokens = match Tokenizer::tokenize(program) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    println!("Tokens: {:?}", tokens);

    let program = match ParserState::parse(tokens) {
        Ok(program) => program,
        Err(e) => {
            println!("Parser error: {:?}", e);
            return;
        }
    };
    println!("Program: {:?}", program);
}

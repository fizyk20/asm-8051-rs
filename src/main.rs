extern crate regex;
mod parser;
use parser::ast::ParserState;
use parser::lexer::Tokenizer;

fn main() {
    let program = "db \"foobar\", 0\nmov P3, #0AAh\nret\n";
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

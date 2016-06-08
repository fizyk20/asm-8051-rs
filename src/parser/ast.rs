use std::fmt::Display;
use std::error;
use super::lexer;
use super::keywords::{Operator, Register};

#[derive(Clone, Debug)]
pub struct Program(Vec<Line>);

#[derive(Clone, Debug)]
pub struct Line(Option<Label>, Option<LineBody>);

#[derive(Clone, Debug)]
pub enum LineBody {
    CodeLine(Operator, Vec<Operand>),
    ValueDefinition(Vec<Value>)
}

#[derive(Clone, Debug)]
pub struct Label(String);

#[derive(Clone, Debug)]
pub enum Operand {
    Register(Register),
    Direct(u8),
    IndirectReg(Register),
    IndirectSum(Register, Register),
    Immediate(i32)
}

#[derive(Clone, Debug)]
pub enum Value {
    Byte(u8),
    String(String)
}

#[derive(Clone, Debug)]
pub enum ParseError {
    GeneralError
}

pub type Result<T> = ::std::result::Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<lexer::Token>
}

impl Parser {

    fn expect_newline(&mut self) -> Result<()> {
        if !self.tokens[0].is_newline() {
            Err(ParseError::GeneralError)
        }
        else {
            self.tokens.remove(0);
            Ok(())
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut lines = Vec::new();

        while self.tokens.len() > 0 {
            let line = try! { self.parse_line() };
            lines.push(line);
        }

        Ok(Program(lines))
    }

    pub fn parse_line(&mut self) -> Result<Line> {
        let result_label = self.parse_label();
        
        let label;
        if let Ok(parse_result_label) = result_label {
            label = Some(parse_result_label);
        }
        else {
            label = None;
        }

        let result_lbody = self.parse_line_body();
        let lbody;
        if let Ok(parse_result_lbody) = result_lbody {
            lbody = Some(parse_result_lbody);
        }
        else {
            lbody = None;
        }

        try! { self.expect_newline() };
        Ok(Line(label, lbody))
    }

    pub fn parse_label(&mut self) -> Result<Label> {
        Err(ParseError::GeneralError)
    }

    pub fn parse_line_body(&mut self) -> Result<LineBody> {
        Err(ParseError::GeneralError)
    }

}

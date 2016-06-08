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
    tokens: Vec<lexer::Token>,
    position: usize,
    saved_positions: Vec<usize>
}

impl Parser {

    fn current_token(&self) -> &lexer::Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn save_pos(&mut self) {
        self.saved_positions.push(self.position);
    }

    fn rollback(&mut self) {
        if let Some(p) = self.saved_positions.pop() {
            self.position = p;
        }
    }

    fn discard_saved_pos(&mut self) {
        self.saved_positions.pop();
    }

    fn expect_newline(&mut self) -> Result<()> {
        if !self.current_token().is_newline() {
            Err(ParseError::GeneralError)
        }
        else {
            self.advance();
            Ok(())
        }
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut lines = Vec::new();

        while self.position < self.tokens.len() {
            let line = try! { self.parse_line() };
            lines.push(line);
        }

        Ok(Program(lines))
    }

    pub fn parse_line(&mut self) -> Result<Line> {
        self.save_pos();

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

        let newline_result = self.expect_newline();
        if let Err(e) = newline_result {
            self.rollback();
            Err(e)
        }
        else {
            self.discard_saved_pos();
            Ok(Line(label, lbody))
        }
    }

    pub fn parse_label(&mut self) -> Result<Label> {
        Err(ParseError::GeneralError)
    }

    pub fn parse_line_body(&mut self) -> Result<LineBody> {
        Err(ParseError::GeneralError)
    }

}

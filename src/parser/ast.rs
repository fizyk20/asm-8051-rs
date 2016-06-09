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
    UnexpectedEof(lexer::Token),
    ExpectedNewline(lexer::Position),
    ExpectedIdentifier(lexer::Position),
    ExpectedColon(lexer::Position),
    ExpectedComma(lexer::Position),
    InvalidLineBody(lexer::Position),
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

    fn advance(&mut self) -> Result<()> {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
            Ok(())
        }
        else {
            Err(ParseError::UnexpectedEof(self.current_token().clone()))
        }
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
        if self.current_token().is_newline() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedNewline(self.current_token().get_position()))
        }
    }

    fn expect_comma(&mut self) -> Result<()> {
        if self.current_token().is_comma() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedComma(self.current_token().get_position()))
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

    fn parse_line(&mut self) -> Result<Line> {
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

    fn parse_label(&mut self) -> Result<Label> {
        let label_txt =
            if self.current_token().is_identifier() {
                self.current_token().get_string().unwrap()
            }
            else {
                return Err(ParseError::ExpectedIdentifier(self.current_token().get_position()));
            };
        self.save_pos();

        self.advance();
        if !self.current_token().is_colon() {
            self.rollback();
            return Err(ParseError::ExpectedColon(self.current_token().get_position()));
        }

        self.advance();
        self.discard_saved_pos();
        Ok(Label(label_txt))
    }

    fn parse_line_body(&mut self) -> Result<LineBody> {
        let result = self.parse_code_line();
        if result.is_ok() {
            return result;
        }

        let result = self.parse_value_def();
        if result.is_ok() {
            return result;
        }

        Err(ParseError::InvalidLineBody(self.current_token().get_position()))
    }

    fn parse_code_line(&mut self) -> Result<LineBody> {
        self.save_pos();

        let result_operator = self.parse_operator();
        if let Err(e) = result_operator {
            self.rollback();
            return Err(e);
        }

        let operator = result_operator.unwrap();

        self.discard_saved_pos();
        self.save_pos();

        let mut operands = Vec::new();

        let first_operand = self.parse_operand();

        if let Err(e) = first_operand {
            self.rollback();
            return Err(e);
        }

        operands.push(first_operand.unwrap());

        self.discard_saved_pos();
        self.save_pos();

        while self.expect_comma().is_ok() {
            let next_operand = self.parse_operand();
            if let Err(e) = next_operand {
                self.rollback();
                return Err(e);
            }
            operands.push(next_operand.unwrap());
        }

        self.discard_saved_pos();

        Ok(LineBody::CodeLine(operator, operands))
    }

    fn parse_operator(&mut self) -> Result<Operator> {
        Err(ParseError::GeneralError)
    }

    fn parse_operand(&mut self) -> Result<Operand> {
        Err(ParseError::GeneralError)
    }

    fn parse_value_def(&mut self) -> Result<LineBody> {
        Err(ParseError::GeneralError)
    }

}

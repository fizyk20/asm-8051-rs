use regex::Regex;
use super::lexer;
use super::keywords::{Operator, Register, Direct};

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
    DirectId(Direct),
    IndirectReg(Register),
    IndirectSum(Register, Register),
    Immediate(i32),
    ImmediateId(String)
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
    ExpectedNumber(lexer::Position),
    ExpectedColon(lexer::Position),
    ExpectedComma(lexer::Position),
    ExpectedAt(lexer::Position),
    ExpectedHash(lexer::Position),
    ExpectedPlus(lexer::Position),
    InvalidLineBody(lexer::Position),
    InvalidMnemonic(String, lexer::Position),
    InvalidOperand(lexer::Token),
    InvalidRegister(lexer::Token),
    InvalidDirectId(lexer::Token),
    InvalidNumber(String),
    GeneralError
}

pub type Result<T> = ::std::result::Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    position: usize,
    saved_positions: Vec<usize>
}

impl Parser {

    pub fn parse(tokens: Vec<lexer::Token>) -> Result<Program> {
        let mut parser = Parser {
            tokens: tokens,
            position: 0,
            saved_positions: Vec::new()
        };
        parser.parse_program()
    }

    fn current_token(&self) -> Result<lexer::Token> {
        if self.position < self.tokens.len() {
            Ok(self.tokens[self.position].clone())
        }
        else {
            Err(ParseError::UnexpectedEof(self.tokens[self.tokens.len()-1].clone()))
        }
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
        let cur_tok = self.current_token();
        match cur_tok {
            Err(ParseError::UnexpectedEof(_)) => Ok(()),
            Err(e) => Err(e),
            Ok(tok) => 
                if tok.is_newline() {
                    self.advance();
                    Ok(())
                }
                else {
                    Err(ParseError::ExpectedNewline(try! { self.current_token() }.get_position()))
                }
        }
    }

    fn expect_comma(&mut self) -> Result<()> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_comma() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedComma(cur_tok.get_position()))
        }
    }

    fn expect_at(&mut self) -> Result<()> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_at() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedAt(cur_tok.get_position()))
        }
    }

    fn expect_hash(&mut self) -> Result<()> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_hash() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedHash(cur_tok.get_position()))
        }
    }

    fn expect_plus(&mut self) -> Result<()> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_plus() {
            self.advance();
            Ok(())
        }
        else {
            Err(ParseError::ExpectedPlus(cur_tok.get_position()))
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
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
        let cur_tok = try! { self.current_token() };
        let label_txt =
            if cur_tok.is_identifier() {
                cur_tok.get_string().unwrap()
            }
            else {
                return Err(ParseError::ExpectedIdentifier(cur_tok.get_position()));
            };
        self.save_pos();

        self.advance();
        let cur_tok = self.current_token();
        if let Err(e) = cur_tok {
            self.rollback();
            return Err(e);
        }
        let cur_tok = cur_tok.unwrap();

        if !cur_tok.is_colon() {
            self.rollback();
            return Err(ParseError::ExpectedColon(cur_tok.get_position()));
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

        let cur_tok = try! { self.current_token() };
        Err(ParseError::InvalidLineBody(cur_tok.get_position()))
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

        if let Err(_) = first_operand {
            self.rollback();
            return Ok(LineBody::CodeLine(operator, vec![]));
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
        let cur_tok = try! { self.current_token() };
        if !cur_tok.is_identifier() {
            return Err(ParseError::ExpectedIdentifier(cur_tok.get_position()));
        }

        let oper_str = cur_tok.get_string().unwrap();
        let operator = oper_str.parse();
        if operator.is_err() {
            return Err(ParseError::InvalidMnemonic(oper_str, cur_tok.get_position()));
        }

        self.advance();
        Ok(operator.unwrap())
    }

    fn parse_base(s: &str, base: u8) -> i32 {
        let negative = s.starts_with("-");
        let unsigned_s = if negative { &s[1..] } else { &s[..] };
        let mut result: i32 = 0;
        for c in unsigned_s.chars() {
            let digit = c.to_digit(base as u32).unwrap();
            result *= base as i32;
            result += digit as i32;
        }
        if negative {
            -result
        }
        else {
            result
        }
    }

    fn parse_number(&mut self) -> Result<i32> {
        let cur_tok = try! { self.current_token() };
        if !cur_tok.is_number() {
            return Err(ParseError::ExpectedNumber(cur_tok.get_position()));
        }

        let num_string = cur_tok.get_string().unwrap().to_lowercase();
        let dec_re = Regex::new(r"^(-?[0-9])+$").unwrap();
        let bin_re = Regex::new(r"^(-?[01])+b$").unwrap();
        let hex_re = Regex::new(r"^(-?[0-9][0-9a-f]*)h$").unwrap();
        let oct_re = Regex::new(r"^(-?[0-7]+)o$").unwrap();

        if let Some(caps) = bin_re.captures(&num_string) {
            self.advance();
            return Ok(Parser::parse_base(caps.at(1).unwrap(), 2));
        }
        else if let Some(caps) = oct_re.captures(&num_string) {
            self.advance();
            return Ok(Parser::parse_base(caps.at(1).unwrap(), 8));
        }
        else if let Some(caps) = dec_re.captures(&num_string) {
            self.advance();
            return Ok(Parser::parse_base(caps.at(1).unwrap(), 10));
        }
        else if let Some(caps) = hex_re.captures(&num_string) {
            self.advance();
            return Ok(Parser::parse_base(caps.at(1).unwrap(), 16));
        }

        Err(ParseError::InvalidNumber(num_string))
    }

    fn parse_operand(&mut self) -> Result<Operand> {
        let res_indirect_sum = self.parse_indirect_sum();
        if let Ok(result) = res_indirect_sum {
            return Ok(result);
        }

        let res_indirect = self.parse_indirect();
        if let Ok(result) = res_indirect {
            return Ok(result);
        }

        let res_immediate = self.parse_immediate();
        if let Ok(result) = res_immediate {
            return Ok(result);
        }

        let res_register = self.parse_register();
        if let Ok(result) = res_register {
            return Ok(Operand::Register(result));
        }

        let res_direct = self.parse_direct();
        if let Ok(result) = res_direct {
            return Ok(result);
        }

        let cur_tok = try! { self.current_token() };
        Err(ParseError::InvalidOperand(cur_tok))
    }
    
    fn parse_indirect_sum(&mut self) -> Result<Operand> {
        self.save_pos();
        if let Err(e) = self.expect_at() {
            self.rollback();
            return Err(e);
        }
        let register1 = self.parse_register();
        if let Err(e) = register1 {
            self.rollback();
            return Err(e);
        }
        let register1 = register1.unwrap();

        if let Err(e) = self.expect_plus() {
            self.rollback();
            return Err(e);
        }

        let register2 = self.parse_register();
        if let Err(e) = register2 {
            self.rollback();
            return Err(e);
        }
        let register2 = register2.unwrap();

        self.discard_saved_pos();
        Ok(Operand::IndirectSum(register1, register2))
    }
    
    fn parse_indirect(&mut self) -> Result<Operand> {
        self.save_pos();
        if let Err(e) = self.expect_at() {
            self.rollback();
            return Err(e);
        }
        let register = self.parse_register();
        if let Err(e) = register {
            self.rollback();
            return Err(e);
        }

        let register = register.unwrap();
        self.discard_saved_pos();
        Ok(Operand::IndirectReg(register))
    }
    
    fn parse_immediate(&mut self) -> Result<Operand> {
        self.save_pos();
        if let Err(e) = self.expect_hash() {
            self.rollback();
            return Err(e);
        }

        let cur_tok = self.current_token();
        if let Err(e) = cur_tok {
            self.rollback();
            return Err(e);
        }
        let cur_tok = cur_tok.unwrap();

        if cur_tok.is_identifier() {
            self.discard_saved_pos();
            self.advance();
            return Ok(Operand::ImmediateId(cur_tok.get_string().unwrap()));
        }
        
        if cur_tok.is_number() {
            let number = self.parse_number();
            if let Err(e) = number {
                self.rollback();
                return Err(e);
            }
            self.discard_saved_pos();
            return Ok(Operand::Immediate(number.unwrap()));
        }

        self.rollback();
        Err(ParseError::GeneralError)
    }
    
    fn parse_register(&mut self) -> Result<Register> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_identifier() {
            if let Ok(reg_result) = cur_tok.get_string().unwrap().parse() {
                self.advance();
                Ok(reg_result)
            }
            else {
                Err(ParseError::InvalidRegister(cur_tok))
            }
        }
        else {
            Err(ParseError::ExpectedIdentifier(cur_tok.get_position()))
        }
    }

    fn parse_direct(&mut self) -> Result<Operand> {
        let cur_tok = try! { self.current_token() };
        if cur_tok.is_identifier() {
            if let Ok(dir_result) = cur_tok.get_string().unwrap().parse() {
                self.advance();
                Ok(Operand::DirectId(dir_result))
            }
            else {
                Err(ParseError::InvalidDirectId(cur_tok))
            }
        }
        else if cur_tok.is_number() {
            let number = try! { self.parse_number() };
            Ok(Operand::Direct(number as u8))
        }
        else {
            Err(ParseError::ExpectedIdentifier(cur_tok.get_position()))
        }
    }

    fn parse_value_def(&mut self) -> Result<LineBody> {
        Err(ParseError::GeneralError)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::lexer::Tokenizer;

    #[test]
    fn test_parser() {
        let program = "mov A, 20h\nret";
        let tokens = Tokenizer::tokenize(program);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        println!("{:?}", tokens);

        let parsed_program = Parser::parse(tokens);
        assert!(parsed_program.is_ok());
        println!("{:?}", parsed_program);
    }
}

use super::keywords::{Definition, Direct, Operator, Register};
use super::lexer;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Program(Vec<Line>);

#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    OrgLine(u16),
    ProgramLine(Option<Label>, Option<LineBody>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineBody {
    CodeLine(Operator, Vec<Operand>),
    ValueDefinition(Vec<Value>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label(String);

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Register(Register),
    Direct(u8),
    DirectId(Direct),
    IndirectReg(Register),
    IndirectSum(Register, Register),
    Immediate(i32),
    ImmediateId(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Byte(u8),
    Word(u16),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
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
    InvalidByte(i32),
    InvalidWord(i32),
    GeneralError,
}

pub type Result<T> = ::std::result::Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParserState<'a> {
    tokens: &'a Vec<lexer::Token>,
    position: usize,
}

pub struct ParseResult<'a, T> {
    state: ParserState<'a>,
    result: T,
}

impl<'a> ParserState<'a> {
    pub fn new<'b>(tokens: &'b Vec<lexer::Token>) -> ParserState<'b> {
        ParserState {
            tokens: tokens,
            position: 0,
        }
    }

    pub fn parse(tokens: Vec<lexer::Token>) -> Result<Program> {
        let parser = ParserState::new(&tokens);
        let result = parser.parse_program()?;
        Ok(result.result)
    }

    fn current_token(&self) -> Result<lexer::Token> {
        if self.position < self.tokens.len() {
            Ok(self.tokens[self.position].clone())
        } else {
            Err(ParseError::UnexpectedEof(self.tokens[self.tokens.len() - 1].clone()))
        }
    }

    fn advanced(self) -> ParserState<'a> {
        ParserState {
            tokens: self.tokens,
            position: self.position + 1,
        }
    }

    fn expect_newline(self) -> Result<ParserState<'a>> {
        let cur_tok = self.current_token();
        match cur_tok {
            Err(ParseError::UnexpectedEof(_)) => Ok(self),
            Err(e) => Err(e),
            Ok(tok) => {
                if tok.is_newline() {
                    Ok(self.advanced())
                } else {
                    Err(ParseError::ExpectedNewline(self.current_token()?.get_position()))
                }
            }
        }
    }

    fn expect_comma(self) -> Result<ParserState<'a>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_comma() {
            Ok(self.advanced())
        } else {
            Err(ParseError::ExpectedComma(cur_tok.get_position()))
        }
    }

    fn expect_at(self) -> Result<ParserState<'a>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_at() {
            Ok(self.advanced())
        } else {
            Err(ParseError::ExpectedAt(cur_tok.get_position()))
        }
    }

    fn expect_hash(self) -> Result<ParserState<'a>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_hash() {
            Ok(self.advanced())
        } else {
            Err(ParseError::ExpectedHash(cur_tok.get_position()))
        }
    }

    fn expect_plus(self) -> Result<ParserState<'a>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_plus() {
            Ok(self.advanced())
        } else {
            Err(ParseError::ExpectedPlus(cur_tok.get_position()))
        }
    }

    fn parse_program(self) -> Result<ParseResult<'a, Program>> {
        let mut lines = Vec::new();

        let mut cur_state = self;

        while cur_state.position < cur_state.tokens.len() {
            let result = cur_state.parse_line()?;
            cur_state = result.state;
            lines.push(result.result);
        }

        Ok(ParseResult {
               state: cur_state,
               result: Program(lines),
           })
    }

    fn parse_line(self) -> Result<ParseResult<'a, Line>> {
        let mut cur_state = self;

        if cur_state.current_token()?.is_identifier() &&
           cur_state.current_token()?.get_string().unwrap() == "org" {
            cur_state = cur_state.advanced();
            let ParseResult {
                state: cur_state,
                result: number,
            } = cur_state.parse_number()?;
            return Ok(ParseResult {
                          state: cur_state,
                          result: Line::OrgLine(Self::to_word(number)?),
                      });
        }

        let result_label = cur_state.clone().parse_label();

        let label;
        if let Ok(parse_result_label) = result_label {
            cur_state = parse_result_label.state;
            label = Some(parse_result_label.result);
        } else {
            label = None;
        }

        let result_lbody = cur_state.clone().parse_line_body();
        let lbody;
        if let Ok(parse_result_lbody) = result_lbody {
            cur_state = parse_result_lbody.state;
            lbody = Some(parse_result_lbody.result);
        } else {
            lbody = None;
        }

        let newline_result = cur_state.expect_newline()?;
        Ok(ParseResult {
               state: newline_result,
               result: Line::ProgramLine(label, lbody),
           })
    }

    fn parse_label(self) -> Result<ParseResult<'a, Label>> {
        let cur_tok = self.current_token()?;
        let label_txt = if cur_tok.is_identifier() {
            cur_tok.get_string().unwrap()
        } else {
            return Err(ParseError::ExpectedIdentifier(cur_tok.get_position()));
        };

        let cur_state = self.advanced();
        let cur_tok = cur_state.current_token()?;

        if !cur_tok.is_colon() {
            return Err(ParseError::ExpectedColon(cur_tok.get_position()));
        }

        Ok(ParseResult {
               state: cur_state.advanced(),
               result: Label(label_txt),
           })
    }

    fn parse_line_body(self) -> Result<ParseResult<'a, LineBody>> {
        let result = self.clone().parse_code_line();
        if result.is_ok() {
            return result;
        }

        let result = self.clone().parse_value_def();
        if result.is_ok() {
            return result;
        }

        let cur_tok = self.current_token()?;
        Err(ParseError::InvalidLineBody(cur_tok.get_position()))
    }

    fn parse_code_line(self) -> Result<ParseResult<'a, LineBody>> {
        let ParseResult {
            state: cur_state,
            result: operator,
        } = self.parse_operator()?;

        let mut operands = Vec::new();

        let first_operand = cur_state.clone().parse_operand();

        if let Err(_) = first_operand {
            return Ok(ParseResult {
                          state: cur_state,
                          result: LineBody::CodeLine(operator, vec![]),
                      });
        }

        let ParseResult {
            state: mut cur_state,
            result: first_operand,
        } = first_operand.unwrap();
        operands.push(first_operand);

        while let Ok(new_state) = cur_state.clone().expect_comma() {
            let ParseResult {
                state: new_state2,
                result: next_operand,
            } = new_state.parse_operand()?;
            cur_state = new_state2;
            operands.push(next_operand);
        }

        Ok(ParseResult {
               state: cur_state,
               result: LineBody::CodeLine(operator, operands),
           })
    }

    fn parse_operator(self) -> Result<ParseResult<'a, Operator>> {
        let cur_tok = self.current_token()?;
        if !cur_tok.is_identifier() {
            return Err(ParseError::ExpectedIdentifier(cur_tok.get_position()));
        }

        let oper_str = cur_tok.get_string().unwrap();
        let operator = oper_str.parse();
        if operator.is_err() {
            return Err(ParseError::InvalidMnemonic(oper_str, cur_tok.get_position()));
        }

        Ok(ParseResult {
               state: self.advanced(),
               result: operator.unwrap(),
           })
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
        if negative { -result } else { result }
    }

    fn parse_number(self) -> Result<ParseResult<'a, i32>> {
        let cur_tok = self.current_token()?;
        if !cur_tok.is_number() {
            return Err(ParseError::ExpectedNumber(cur_tok.get_position()));
        }

        let num_string = cur_tok.get_string().unwrap().to_lowercase();
        let dec_re = Regex::new(r"^(-?[0-9]+)$").unwrap();
        let bin_re = Regex::new(r"^(-?[01]+)b$").unwrap();
        let hex_re = Regex::new(r"^(-?[0-9][0-9a-f]*)h$").unwrap();
        let oct_re = Regex::new(r"^(-?[0-7]+)o$").unwrap();

        if let Some(caps) = bin_re.captures(&num_string) {
            return Ok(ParseResult {
                          state: self.advanced(),
                          result: ParserState::parse_base(caps.at(1).unwrap(), 2),
                      });
        } else if let Some(caps) = oct_re.captures(&num_string) {
            return Ok(ParseResult {
                          state: self.advanced(),
                          result: ParserState::parse_base(caps.at(1).unwrap(), 8),
                      });
        } else if let Some(caps) = dec_re.captures(&num_string) {
            return Ok(ParseResult {
                          state: self.advanced(),
                          result: ParserState::parse_base(caps.at(1).unwrap(), 10),
                      });
        } else if let Some(caps) = hex_re.captures(&num_string) {
            return Ok(ParseResult {
                          state: self.advanced(),
                          result: ParserState::parse_base(caps.at(1).unwrap(), 16),
                      });
        }

        Err(ParseError::InvalidNumber(num_string))
    }

    fn parse_operand(self) -> Result<ParseResult<'a, Operand>> {
        let res_indirect_sum = self.clone().parse_indirect_sum();
        if res_indirect_sum.is_ok() {
            return res_indirect_sum;
        }

        let res_indirect = self.clone().parse_indirect();
        if res_indirect.is_ok() {
            return res_indirect;
        }

        let res_immediate = self.clone().parse_immediate();
        if res_immediate.is_ok() {
            return res_immediate;
        }

        let res_register = self.clone().parse_register();
        if let Ok(result) = res_register {
            return Ok(ParseResult {
                          state: result.state,
                          result: Operand::Register(result.result),
                      });
        }

        let res_direct = self.clone().parse_direct();
        if res_direct.is_ok() {
            return res_direct;
        }

        let cur_tok = self.current_token()?;
        Err(ParseError::InvalidOperand(cur_tok))
    }

    fn parse_indirect_sum(self) -> Result<ParseResult<'a, Operand>> {
        let cur_state = self.expect_at()?;

        let ParseResult {
            state: cur_state,
            result: register1,
        } = cur_state.parse_register()?;

        let cur_state = cur_state.expect_plus()?;

        let ParseResult {
            state: cur_state,
            result: register2,
        } = cur_state.parse_register()?;

        Ok(ParseResult {
               state: cur_state,
               result: Operand::IndirectSum(register1, register2),
           })
    }

    fn parse_indirect(self) -> Result<ParseResult<'a, Operand>> {
        let cur_state = self.expect_at()?;

        let ParseResult {
            state: cur_state,
            result: register,
        } = cur_state.parse_register()?;

        Ok(ParseResult {
               state: cur_state,
               result: Operand::IndirectReg(register),
           })
    }

    fn parse_immediate(self) -> Result<ParseResult<'a, Operand>> {
        let cur_state = self.expect_hash()?;

        let cur_tok = cur_state.current_token()?;

        if cur_tok.is_identifier() {
            return Ok(ParseResult {
                          state: cur_state.advanced(),
                          result: Operand::ImmediateId(cur_tok.get_string().unwrap()),
                      });
        }

        if cur_tok.is_number() {
            let ParseResult {
                state: cur_state,
                result: number,
            } = cur_state.parse_number()?;
            return Ok(ParseResult {
                          state: cur_state,
                          result: Operand::Immediate(number),
                      });
        }

        Err(ParseError::GeneralError)
    }

    fn parse_register(self) -> Result<ParseResult<'a, Register>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_identifier() {
            if let Ok(reg_result) = cur_tok.get_string().unwrap().parse() {
                Ok(ParseResult {
                       state: self.advanced(),
                       result: reg_result,
                   })
            } else {
                Err(ParseError::InvalidRegister(cur_tok))
            }
        } else {
            Err(ParseError::ExpectedIdentifier(cur_tok.get_position()))
        }
    }

    fn parse_direct(self) -> Result<ParseResult<'a, Operand>> {
        let cur_tok = self.current_token()?;
        if cur_tok.is_identifier() {
            if let Ok(dir_result) = cur_tok.get_string().unwrap().parse() {
                Ok(ParseResult {
                       state: self.advanced(),
                       result: Operand::DirectId(dir_result),
                   })
            } else {
                Err(ParseError::InvalidDirectId(cur_tok))
            }
        } else if cur_tok.is_number() {
            let number = self.parse_number()?;
            Ok(ParseResult {
                   state: number.state,
                   result: Operand::Direct(number.result as u8),
               })
        } else {
            Err(ParseError::ExpectedIdentifier(cur_tok.get_position()))
        }
    }

    fn parse_value_def(self) -> Result<ParseResult<'a, LineBody>> {
        let ParseResult {
            state: cur_state,
            result: definition,
        } = self.parse_definition()?;

        let ParseResult {
            state: cur_state,
            result: values,
        } = match definition {
            Definition::DefineByte => cur_state.parse_bytes()?,
            Definition::DefineWord => cur_state.parse_words()?,
        };

        Ok(ParseResult {
               state: cur_state,
               result: LineBody::ValueDefinition(values),
           })
    }

    fn parse_definition(self) -> Result<ParseResult<'a, Definition>> {
        let cur_tok = self.current_token()?;
        if !cur_tok.is_identifier() {
            return Err(ParseError::ExpectedIdentifier(cur_tok.get_position()));
        }

        let def_str = cur_tok.get_string().unwrap();
        let definition = def_str.parse();
        if definition.is_err() {
            return Err(ParseError::InvalidMnemonic(def_str, cur_tok.get_position()));
        }

        Ok(ParseResult {
               state: self.advanced(),
               result: definition.unwrap(),
           })
    }

    fn parse_bytes(self) -> Result<ParseResult<'a, Vec<Value>>> {
        let mut values = Vec::new();

        let ParseResult {
            state: mut cur_state,
            result: first_value,
        } = self.parse_byte_value()?;

        values.push(first_value);

        while let Ok(new_state) = cur_state.clone().expect_comma() {
            let ParseResult {
                state: new_state2,
                result: next_value,
            } = new_state.parse_byte_value()?;
            cur_state = new_state2;
            values.push(next_value);
        }

        Ok(ParseResult {
               state: cur_state,
               result: values,
           })
    }

    fn parse_words(self) -> Result<ParseResult<'a, Vec<Value>>> {
        let mut values = Vec::new();

        let ParseResult {
            state: mut cur_state,
            result: first_number,
        } = self.parse_number()?;

        values.push(Value::Word(Self::to_word(first_number)?));

        while let Ok(new_state) = cur_state.clone().expect_comma() {
            let ParseResult {
                state: new_state2,
                result: next_value,
            } = new_state.parse_number()?;
            cur_state = new_state2;
            values.push(Value::Word(Self::to_word(next_value)?));
        }

        Ok(ParseResult {
               state: cur_state,
               result: values,
           })
    }

    fn parse_byte_value(self) -> Result<ParseResult<'a, Value>> {
        let cur_tok = self.current_token()?;
        if let lexer::Token::String(s, _) = cur_tok {
            return Ok(ParseResult {
                          state: self.advanced(),
                          result: Value::String(s),
                      });
        }

        let ParseResult {
            state: cur_state,
            result: byte,
        } = self.parse_number()?;

        Ok(ParseResult {
               state: cur_state,
               result: Value::Byte(Self::to_byte(byte)?),
           })
    }

    fn to_byte(byte: i32) -> Result<u8> {
        if byte >= 0 && byte <= 255 {
            Ok(byte as u8)
        } else {
            Err(ParseError::InvalidByte(byte))
        }
    }

    fn to_word(word: i32) -> Result<u16> {
        if word >= 0 && word <= 65535 {
            Ok(word as u16)
        } else {
            Err(ParseError::InvalidWord(word))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::keywords::*;
    use super::super::lexer::{Token, Tokenizer};

    fn tokens(s: &str) -> Vec<Token> {
        Tokenizer::tokenize(s).unwrap()
    }

    // Operands tests

    // Registers

    #[test]
    fn test_parse_register() {
        let tokens = tokens("R1");
        let state = ParserState::new(&tokens);
        let result = state.parse_register();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, Register::R(1));
    }

    #[test]
    fn test_parse_register_fail() {
        let tokens = tokens("R8");
        let state = ParserState::new(&tokens);
        let result = state.parse_register();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_register_fail2() {
        let tokens = tokens("asdf");
        let state = ParserState::new(&tokens);
        let result = state.parse_register();
        assert!(result.is_err());
    }

    // Immediate values

    #[test]
    fn test_number_binary() {
        let tokens = tokens("01101001b");
        let state = ParserState::new(&tokens);
        let result = state.parse_number();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, 105);
    }

    #[test]
    fn test_number_octal() {
        let tokens = tokens("744o");
        let state = ParserState::new(&tokens);
        let result = state.parse_number();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, 484);
    }

    #[test]
    fn test_number_decimal() {
        let tokens = tokens("183");
        let state = ParserState::new(&tokens);
        let result = state.parse_number();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, 183);
    }

    #[test]
    fn test_number_hex() {
        let tokens = tokens("0cdh");
        let state = ParserState::new(&tokens);
        let result = state.parse_number();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, 205);
    }

    #[test]
    fn test_number_invalid() {
        let tokens = tokens("0cdo");
        let state = ParserState::new(&tokens);
        let result = state.parse_number();
        assert!(result.is_err());
    }

    #[test]
    fn test_immediate() {
        let tokens = tokens("#0ABh");
        let state = ParserState::new(&tokens);
        let result = state.parse_immediate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, Operand::Immediate(171));
    }

    #[test]
    fn test_immediate_id() {
        let tokens = tokens("#label");
        let state = ParserState::new(&tokens);
        let result = state.parse_immediate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result,
                   Operand::ImmediateId("label".to_string()));
    }

    #[test]
    fn test_define_bytes() {
        let tokens = tokens("db \"foo bar quux\", 13, 10, 0");
        let state = ParserState::new(&tokens);
        let result = state.parse_value_def();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result,
                   LineBody::ValueDefinition(vec![Value::String("foo bar quux".to_owned()),
                                                  Value::Byte(13),
                                                  Value::Byte(10),
                                                  Value::Byte(0)]));
    }

    #[test]
    fn test_define_words() {
        let tokens = tokens("dw 278, 10765, 13");
        let state = ParserState::new(&tokens);
        let result = state.parse_value_def();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result,
                   LineBody::ValueDefinition(vec![Value::Word(278),
                                                  Value::Word(10765),
                                                  Value::Word(13)]));
    }

    #[test]
    fn test_invalid_bytes() {
        let tokens = tokens("db \"foo bar quux\", 1376, 10, 0");
        let state = ParserState::new(&tokens);
        let result = state.parse_value_def();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_words() {
        let tokens = tokens("dw 278, \"abc\", 13");
        let state = ParserState::new(&tokens);
        let result = state.parse_value_def();
        assert!(result.is_err());
    }

    #[test]
    fn test_parser() {
        let program = "test: db \"foobar\", 0 ; test\nmov A, 20h\nret";
        let tokens = Tokenizer::tokenize(program);
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        println!("{:?}", tokens);

        let parsed_program = ParserState::parse(tokens).unwrap();
        //assert!(parsed_program.is_ok());
        println!("{:?}", parsed_program);
    }
}

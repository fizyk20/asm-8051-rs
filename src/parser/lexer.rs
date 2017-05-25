use super::keywords::{DirectLocation, Keyword, Operator};
use std::error;
use std::fmt::Display;
use std::io::{self, Write};
use std::mem;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    row: usize,
    column: usize,
}

/// An enumeration listing possible tokens
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Identifier(String, Position),
    Operator(Operator, Position),
    DirectLocation(DirectLocation, Position),
    Keyword(Keyword, Position),
    Number(String, Position),
    String(String, Position),
    Colon(Position),
    Comma(Position),
    Dot(Position),
    At(Position),
    Plus(Position),
    Newline(Position),
    LeftBracket(Position),
    RightBracket(Position),
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        match *self {
            Token::Identifier(_, _) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Token::Number(_, _) => true,
            _ => false,
        }
    }

    pub fn is_colon(&self) -> bool {
        match *self {
            Token::Colon(_) => true,
            _ => false,
        }
    }

    pub fn is_comma(&self) -> bool {
        match *self {
            Token::Comma(_) => true,
            _ => false,
        }
    }

    pub fn is_dot(&self) -> bool {
        match *self {
            Token::Dot(_) => true,
            _ => false,
        }
    }

    pub fn is_at(&self) -> bool {
        match *self {
            Token::At(_) => true,
            _ => false,
        }
    }

    pub fn is_plus(&self) -> bool {
        match *self {
            Token::Plus(_) => true,
            _ => false,
        }
    }

    pub fn is_left_bracket(&self) -> bool {
        match *self {
            Token::LeftBracket(_) => true,
            _ => false,
        }
    }

    pub fn is_right_bracket(&self) -> bool {
        match *self {
            Token::RightBracket(_) => true,
            _ => false,
        }
    }

    pub fn is_newline(&self) -> bool {
        match *self {
            Token::Newline(_) => true,
            _ => false,
        }
    }

    pub fn get_position(&self) -> Position {
        match *self {
            Token::Identifier(_, p) => p,
            Token::Operator(_, p) => p,
            Token::DirectLocation(_, p) => p,
            Token::Keyword(_, p) => p,
            Token::Number(_, p) => p,
            Token::String(_, p) => p,
            Token::Colon(p) => p,
            Token::Comma(p) => p,
            Token::Dot(p) => p,
            Token::At(p) => p,
            Token::Plus(p) => p,
            Token::Newline(p) => p,
            Token::LeftBracket(p) => p,
            Token::RightBracket(p) => p,
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match *self {
            Token::Identifier(ref s, _) => Some(s.clone()),
            Token::Number(ref s, _) => Some(s.clone()),
            _ => None,
        }
    }
}

/// Possible tokenizer states
#[derive(Debug)]
enum TokenizerState {
    Ready,
    ReadingNumber(Vec<char>, Position),
    ReadingIdentifier(Vec<char>, Position),
    ReadingString(Vec<char>, Position),
    ReadingStringEscape(Vec<char>, Position),
    ReadingComment,
    Invalid,
}

/// The tokenizer struct
pub struct Tokenizer {
    tokens: Vec<Token>,
    cur_pos: Position,
    state: TokenizerState,
}

#[derive(Debug)]
pub enum TokenizerError {
    UnexpectedCharacter(Position, char),
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TokenizerError::UnexpectedCharacter(pos, c) => {
                write!(f,
                       "Unexpected character {} at row {}, column {}",
                       c,
                       pos.row,
                       pos.column)
            }
        }
    }
}

impl error::Error for TokenizerError {
    fn description(&self) -> &str {
        match *self {
            TokenizerError::UnexpectedCharacter(_, _) => "unexpected character in tokenized stream",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            cur_pos: Position { row: 1, column: 1 },
            state: TokenizerState::Ready,
        }
    }

    fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn advance(&mut self) {
        self.cur_pos.column += 1;
    }

    fn newline(&mut self) {
        self.cur_pos.column = 1;
        self.cur_pos.row += 1;
    }

    fn handle_ready(&mut self, c: char) -> Result<(), TokenizerError> {
        match c {
            ' ' | '\t' | '\r' => {
                self.state = TokenizerState::Ready;
                self.advance();
                Ok(())
            }

            '\n' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Newline(self.cur_pos));
                self.advance();
                self.newline();
                Ok(())
            }

            '0'...'9' => {
                self.state = TokenizerState::ReadingNumber(vec![c], self.cur_pos);
                self.advance();
                Ok(())
            }

            '_' | 'a'...'z' | 'A'...'Z' => {
                self.state = TokenizerState::ReadingIdentifier(vec![c], self.cur_pos);
                self.advance();
                Ok(())
            }

            ':' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Colon(self.cur_pos));
                self.advance();
                Ok(())
            }

            ',' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Comma(self.cur_pos));
                self.advance();
                Ok(())
            }

            '.' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Dot(self.cur_pos));
                self.advance();
                Ok(())
            }

            '@' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::At(self.cur_pos));
                self.advance();
                Ok(())
            }

            '+' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Plus(self.cur_pos));
                self.advance();
                Ok(())
            }

            '[' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::LeftBracket(self.cur_pos));
                self.advance();
                Ok(())
            }

            ']' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::RightBracket(self.cur_pos));
                self.advance();
                Ok(())
            }

            ';' => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                Ok(())
            }

            '"' => {
                self.state = TokenizerState::ReadingString(vec![], self.cur_pos);
                self.advance();
                Ok(())
            }

            _ => {
                self.state = TokenizerState::Ready;
                Err(TokenizerError::UnexpectedCharacter(self.cur_pos, c))
            }
        }
    }

    fn handle_comment(&mut self, c: char) -> Result<(), TokenizerError> {
        match c {
            '\n' => {
                self.tokens.push(Token::Newline(self.cur_pos));
                self.state = TokenizerState::Ready;
                self.advance();
                self.newline();
                Ok(())
            }

            _ => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                Ok(())
            }
        }
    }

    fn handle_number(&mut self,
                     mut v: Vec<char>,
                     p: Position,
                     c: char)
                     -> Result<(), TokenizerError> {
        match c {
            '0'...'9' | 'a'...'f' | 'A'...'F' | 'h' | 'H' | 'o' | 'O' => {
                v.push(c);
                self.state = TokenizerState::ReadingNumber(v, p);
                self.advance();
                Ok(())
            }

            ' ' | '\t' | '\r' | '\n' | ',' | '+' | '-' | '*' | '/' | ';' => {
                self.tokens
                    .push(Token::Number(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                self.consume_char(c)
            }

            _ => {
                self.state = TokenizerState::ReadingNumber(v, p);
                Err(TokenizerError::UnexpectedCharacter(self.cur_pos, c))
            }
        }
    }

    fn categorize_identifier(v: Vec<char>, p: Position) -> Token {
        let s: String = v.into_iter().collect();
        if let Ok(kw) = s.parse::<Keyword>() {
            Token::Keyword(kw, p)
        } else if let Ok(dir) = s.parse::<DirectLocation>() {
            Token::DirectLocation(dir, p)
        } else if let Ok(oper) = s.parse::<Operator>() {
            Token::Operator(oper, p)
        } else {
            Token::Identifier(s, p)
        }
    }

    fn handle_identifier(&mut self,
                         mut v: Vec<char>,
                         p: Position,
                         c: char)
                         -> Result<(), TokenizerError> {
        match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {
                v.push(c);
                self.state = TokenizerState::ReadingIdentifier(v, p);
                self.advance();
                Ok(())
            }

            _ => {
                self.tokens.push(Self::categorize_identifier(v, p));
                self.state = TokenizerState::Ready;
                self.consume_char(c)
            }
        }
    }

    fn handle_string(&mut self,
                     mut v: Vec<char>,
                     p: Position,
                     c: char)
                     -> Result<(), TokenizerError> {
        match c {
            '"' => {
                self.tokens
                    .push(Token::String(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                self.advance();
                Ok(())
            }
            '\\' => {
                self.state = TokenizerState::ReadingStringEscape(v, p);
                self.advance();
                Ok(())
            }
            '\n' => Err(TokenizerError::UnexpectedCharacter(self.cur_pos, '\n')),
            c => {
                v.push(c);
                self.state = TokenizerState::ReadingString(v, p);
                self.advance();
                Ok(())
            }
        }
    }

    fn handle_string_escape(&mut self,
                            mut v: Vec<char>,
                            p: Position,
                            c: char)
                            -> Result<(), TokenizerError> {
        match c {
            'r' => {
                v.push('\r');
            }
            'n' => {
                v.push('\n');
            }
            't' => {
                v.push('\t');
            }
            '\\' => {
                v.push('\\');
            }
            '"' => {
                v.push('"');
            }
            c => {
                return Err(TokenizerError::UnexpectedCharacter(self.cur_pos, c));
            }
        }
        self.advance();
        self.state = TokenizerState::ReadingString(v, p);
        Ok(())
    }

    pub fn consume_char(&mut self, c: char) -> Result<(), TokenizerError> {
        match mem::replace(&mut self.state, TokenizerState::Invalid) {
            TokenizerState::Ready => self.handle_ready(c),

            TokenizerState::ReadingNumber(v, p) => self.handle_number(v, p, c),

            TokenizerState::ReadingIdentifier(v, p) => self.handle_identifier(v, p, c),

            TokenizerState::ReadingString(v, p) => self.handle_string(v, p, c),

            TokenizerState::ReadingStringEscape(v, p) => self.handle_string_escape(v, p, c),

            TokenizerState::ReadingComment => self.handle_comment(c),

            TokenizerState::Invalid => panic!("Tokenizer caught in invalid state"),
        }
    }

    pub fn consume_text(&mut self, text: &str) -> Result<(), TokenizerError> {
        for c in text.chars() {
            self.consume_char(c)?;
        }
        Ok(())
    }

    pub fn tokenize(text: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut tokenizer = Tokenizer::new();
        tokenizer.consume_text(text)?;
        tokenizer.flush().expect("Tokenizer.flush() failed");
        Ok(tokenizer.get_tokens())
    }
}

impl Write for Tokenizer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let chars = str::from_utf8(buf);
        if let Err(e) = chars {
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }
        let text = chars.unwrap();
        let result = self.consume_text(text);
        if let Err(e) = result {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        match mem::replace(&mut self.state, TokenizerState::Invalid) {
            TokenizerState::Ready => {
                self.state = TokenizerState::Ready;
            }

            TokenizerState::ReadingNumber(v, p) => {
                self.tokens
                    .push(Token::Number(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
            }

            TokenizerState::ReadingIdentifier(v, p) => {
                self.tokens.push(Self::categorize_identifier(v, p));
                self.state = TokenizerState::Ready;
            }

            TokenizerState::ReadingString(v, p) |
            TokenizerState::ReadingStringEscape(v, p) => {
                self.tokens
                    .push(Token::String(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
            }

            TokenizerState::ReadingComment => {
                self.state = TokenizerState::Ready;
            }

            TokenizerState::Invalid => panic!("Tokenizer caught in invalid state"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let text = "label: operator operand1, 0EFh ; comment";
        if let Ok(result) = Tokenizer::tokenize(text) {
            assert_eq!(result.len(), 6);

            // label
            if let Token::Identifier(ref s, pos) = result[0] {
                assert_eq!(s, "label");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[0]: expected Identifier, found {:?}", result[0]);
            }

            // :
            if let Token::Colon(pos) = result[1] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 6);
            } else {
                panic!("result[1]: expected Colon, found {:?}", result[1]);
            }

            // operator
            if let Token::Identifier(ref s, pos) = result[2] {
                assert_eq!(s, "operator");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 8);
            } else {
                panic!("result[2]: expected Identifier, found {:?}", result[2]);
            }

            // operand1
            if let Token::Identifier(ref s, pos) = result[3] {
                assert_eq!(s, "operand1");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 17);
            } else {
                panic!("result[3]: expected Identifier, found {:?}", result[3]);
            }

            // ,
            if let Token::Comma(pos) = result[4] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 25);
            } else {
                panic!("result[4]: expected Comma, found {:?}", result[4]);
            }

            // 0EFh
            if let Token::Number(ref s, pos) = result[5] {
                assert_eq!(s, "0EFh");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 27);
            } else {
                panic!("result[5]: expected Number, found {:?}", result[5]);
            }
        } else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_tokenize2() {
        let text = "mov a, 20h\nret";
        if let Ok(result) = Tokenizer::tokenize(text) {
            assert_eq!(result.len(), 6);

            // mov
            if let Token::Operator(oper, pos) = result[0] {
                assert_eq!(oper, Operator::Mov);
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[0]: expected Operator, found {:?}", result[0]);
            }

            // a
            if let Token::Identifier(ref s, pos) = result[1] {
                assert_eq!(s, "a");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 5);
            } else {
                panic!("result[1]: expected Identifier, found {:?}", result[1]);
            }

            // ,
            if let Token::Comma(pos) = result[2] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 6);
            } else {
                panic!("result[2]: expected Comma, found {:?}", result[2]);
            }

            // 20h
            if let Token::Number(ref s, pos) = result[3] {
                assert_eq!(s, "20h");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 8);
            } else {
                panic!("result[3]: expected Number, found {:?}", result[3]);
            }

            // \n
            if let Token::Newline(pos) = result[4] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 11);
            } else {
                panic!("result[4]: expected Newline, found {:?}", result[4]);
            }

            // ret
            if let Token::Operator(oper, pos) = result[5] {
                assert_eq!(oper, Operator::Ret);
                assert_eq!(pos.row, 2);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[5]: expected Operator, found {:?}", result[5]);
            }
        } else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_string() {
        let text = "\"abcdefg\"";
        if let Ok(result) = Tokenizer::tokenize(text) {
            assert_eq!(result.len(), 1);
            if let Token::String(ref s, pos) = result[0] {
                assert_eq!(s, "abcdefg");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[0]: expected String, found {:?}", result[0]);
            }
        } else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_string_escape() {
        let text = "\"abcd\\nefg\"";
        if let Ok(result) = Tokenizer::tokenize(text) {
            assert_eq!(result.len(), 1);
            if let Token::String(ref s, pos) = result[0] {
                assert_eq!(s, "abcd\nefg");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[0]: expected String, found {:?}", result[0]);
            }
        } else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_string_quote() {
        let text = "\"abcd\\\"efg\"";
        if let Ok(result) = Tokenizer::tokenize(text) {
            assert_eq!(result.len(), 1);
            if let Token::String(ref s, pos) = result[0] {
                assert_eq!(s, "abcd\"efg");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 1);
            } else {
                panic!("result[0]: expected String, found {:?}", result[0]);
            }
        } else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_string_invalid_escape() {
        let text = "\"abcd\\yefg\"";
        if let Err(TokenizerError::UnexpectedCharacter(pos, c)) = Tokenizer::tokenize(text) {
            assert_eq!(pos.row, 1);
            assert_eq!(pos.column, 7);
            assert_eq!(c, 'y');
        } else {
            panic!("Tokenization successful!");
        }
    }

    #[test]
    fn test_invalid_number() {
        let text = "0abcdefgh";
        if let Err(TokenizerError::UnexpectedCharacter(pos, c)) = Tokenizer::tokenize(text) {
            assert_eq!(pos.row, 1);
            assert_eq!(pos.column, 8);
            assert_eq!(c, 'g');
        } else {
            panic!("Tokenization successful!");
        }
    }

}

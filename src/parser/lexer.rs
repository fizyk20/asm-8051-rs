use std::io::{self, Write};
use std::str;
use std::mem;
use std::error;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    row: usize,
    column: usize
}

/// An enumeration listing possible tokens
#[derive(Clone, Debug)]
pub enum Token {
    Identifier(String, Position),
    Number(String, Position),
    Colon(Position),
    Comma(Position),
    At(Position),
    Hash(Position),
    Plus(Position),
    Newline(Position)
}

impl Token {

    pub fn is_identifier(&self) -> bool {
        match *self {
            Token::Identifier(_, _) => true,
            _ => false
        }
    }

    pub fn is_number(&self) -> bool {
        match *self {
            Token::Number(_, _) => true,
            _ => false
        }
    }

    pub fn is_colon(&self) -> bool {
        match *self {
            Token::Colon(_) => true,
            _ => false
        }
    }

    pub fn is_newline(&self) -> bool {
        match *self {
            Token::Newline(_) => true,
            _ => false
        }
    }

    pub fn get_position(&self) -> Position {
        match *self {
            Token::Identifier(_, p) => p,
            Token::Number(_, p) => p,
            Token::Colon(p) => p,
            Token::Comma(p) => p,
            Token::At(p) => p,
            Token::Hash(p) => p,
            Token::Plus(p) => p,
            Token::Newline(p) => p
        }
    }

    pub fn get_string(&self) -> Option<String> {
        match *self {
            Token::Identifier(ref s, _) => Some(s.clone()),
            Token::Number(ref s, _) => Some(s.clone()),
            _ => None
        }
    }

}

/// Possible tokenizer states
#[derive(Debug)]
enum TokenizerState {
    Ready,
    ReadingNumber(Vec<char>, Position),
    ReadingIdentifier(Vec<char>, Position),
    ReadingComment,
    Invalid
}

/// The tokenizer struct
pub struct Tokenizer {
    tokens: Vec<Token>,
    cur_pos: Position,
    state: TokenizerState
}

#[derive(Debug)]
pub enum TokenizerError {
    UnexpectedCharacter(Position, char)
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TokenizerError::UnexpectedCharacter(pos, c) =>
                write!(f, "Unexpected character {} at row {}, column {}", c, pos.row, pos.column)
        }
    }
}

impl error::Error for TokenizerError {
    fn description(&self) -> &str {
        match *self {
            TokenizerError::UnexpectedCharacter(_, _) => "unexpected character in tokenized stream"
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
            state: TokenizerState::Ready
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
            },

            '\n' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Newline(self.cur_pos));
                self.advance();
                self.newline();
                Ok(())
            },

            '0' ... '9' => {
                self.state = TokenizerState::ReadingNumber(vec![c], self.cur_pos);
                self.advance();
                Ok(())
            },

            '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                self.state = TokenizerState::ReadingIdentifier(vec![c], self.cur_pos);
                self.advance();
                Ok(())
            },

            ':' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Colon(self.cur_pos));
                self.advance();
                Ok(())
            },

            ',' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Comma(self.cur_pos));
                self.advance();
                Ok(())
            },

            '@' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::At(self.cur_pos));
                self.advance();
                Ok(())
            },

            '#' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Hash(self.cur_pos));
                self.advance();
                Ok(())
            },

            '+' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Plus(self.cur_pos));
                self.advance();
                Ok(())
            },

            ';' => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                Ok(())
            },

            _ => {
                self.state = TokenizerState::Ready;
                Err(TokenizerError::UnexpectedCharacter(self.cur_pos, c))
            }
        }
    }

    fn handle_comment(&mut self, c: char) -> Result<(), TokenizerError> {
        match c {
            '\n' => {
                self.state = TokenizerState::Ready;
                self.advance();
                self.newline();
                Ok(())
            },

            _ => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                Ok(())
            }
        }
    }
    
    fn handle_number(&mut self, mut v: Vec<char>, p: Position, c: char) -> Result<(), TokenizerError> {
        match c {
            '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' | 'h' | 'H' | 'o' | 'O' => {
                v.push(c);
                self.state = TokenizerState::ReadingNumber(v, p);
                self.advance();
                Ok(())
            },
            
            ' ' | '\t' | '\r' | '\n' | ',' | '+' | '-' | '*' | '/' | ';' => {
                self.tokens.push(Token::Number(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                self.consume_char(c)
            },

            _ => {
                self.state = TokenizerState::ReadingNumber(v, p);
                Err(TokenizerError::UnexpectedCharacter(self.cur_pos, c))
            }
        }
    }

    fn handle_identifier(&mut self, mut v: Vec<char>, p: Position, c: char) -> Result<(), TokenizerError> {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => {
                v.push(c);
                self.state = TokenizerState::ReadingIdentifier(v, p);
                self.advance();
                Ok(())
            },

            _ => {
                self.tokens.push(Token::Identifier(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                self.consume_char(c)
            }
        }
    }

    pub fn consume_char(&mut self, c: char) -> Result<(), TokenizerError> {
        match mem::replace(&mut self.state, TokenizerState::Invalid) {
            TokenizerState::Ready => {
                self.handle_ready(c)
            },
            
            TokenizerState::ReadingNumber(v, p) => {
                self.handle_number(v, p, c)
            },

            TokenizerState::ReadingIdentifier(v, p) => {
                self.handle_identifier(v, p, c)
            },

            TokenizerState::ReadingComment => {
                self.handle_comment(c)
            },

            TokenizerState::Invalid => panic!("Tokenizer caught in invalid state")
        }
    }

    pub fn consume_text(&mut self, text: &str) -> Result<(), TokenizerError> {
        for c in text.chars() {
            try! { self.consume_char(c) };
        }
        Ok(())
    }

    pub fn tokenize(text: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut tokenizer = Tokenizer::new();
        try! { tokenizer.consume_text(text) };
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
            }
            else {
                panic!("result[0] is not an Identifier!");
            }

            // :
            if let Token::Colon(pos) = result[1] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 6);
            }
            else {
                panic!("result[1] is not a Colon!");
            }

            // operator
            if let Token::Identifier(ref s, pos) = result[2] {
                assert_eq!(s, "operator");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 8);
            }
            else {
                panic!("result[2] is not an Identifier!");
            }

            // operand1
            if let Token::Identifier(ref s, pos) = result[3] {
                assert_eq!(s, "operand1");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 17);
            }
            else {
                panic!("result[3] is not an Identifier!");
            }

            // ,
            if let Token::Comma(pos) = result[4] {
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 25);
            }
            else {
                panic!("result[4] is not a Comma!");
            }

            // 0EFh
            if let Token::Number(ref s, pos) = result[5] {
                assert_eq!(s, "0EFh");
                assert_eq!(pos.row, 1);
                assert_eq!(pos.column, 27);
            }
            else {
                panic!("result[5] is not a Number!");
            }
        }
        else {
            panic!("Tokenization failed!");
        }
    }

    #[test]
    fn test_invalid_number() {
        let text = "0abcdefgh";
        if let Err(TokenizerError::UnexpectedCharacter(pos, c)) = Tokenizer::tokenize(text) {
            assert_eq!(pos.row, 1);
            assert_eq!(pos.column, 8);
            assert_eq!(c, 'g');
        }
        else {
            panic!("Tokenization successful!");
        }
    }

}

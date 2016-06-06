use std::mem;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    row: usize,
    column: usize
}

/// An enumeration listing possible tokens
#[derive(Debug)]
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
    string: Vec<char>,
    position: usize,
    curPos: Position,
    state: TokenizerState
}

#[derive(Debug)]
pub enum TokenizerError {
    GeneralError,
    UnexpectedCharacter(Position, char)
}

impl Tokenizer {

    pub fn new(s: &str) -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            string: s.chars().collect(),
            position: 0,
            curPos: Position { row: 1, column: 1 },
            state: TokenizerState::Ready
        }
    }

    fn current_char(&self) -> char {
        self.string[self.position]
    }

    fn eof(&self) -> bool {
        self.position >= self.string.len()
    }

    fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn advance(&mut self) {
        self.curPos.column += 1;
        self.position += 1;
    }

    fn newline(&mut self) {
        self.curPos.column = 1;
        self.curPos.row += 1;
    }

    fn handle_ready(&mut self, c: char) -> Option<TokenizerError> {
        match c {
            ' ' | '\t' | '\r' => {
                self.state = TokenizerState::Ready;
                self.advance();
                None
            },

            '\n' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Newline(self.curPos));
                self.advance();
                self.newline();
                None
            },

            '0' ... '9' => {
                self.state = TokenizerState::ReadingNumber(vec![c], self.curPos);
                self.advance();
                None
            },

            '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                self.state = TokenizerState::ReadingIdentifier(vec![c], self.curPos);
                self.advance();
                None
            },

            ':' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Colon(self.curPos));
                self.advance();
                None
            },

            ',' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Comma(self.curPos));
                self.advance();
                None
            },

            '@' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::At(self.curPos));
                self.advance();
                None
            },

            '#' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Hash(self.curPos));
                self.advance();
                None
            },

            '+' => {
                self.state = TokenizerState::Ready;
                self.tokens.push(Token::Plus(self.curPos));
                self.advance();
                None
            },

            ';' => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                None
            },

            _ => {
                self.state = TokenizerState::Ready;
                Some(TokenizerError::UnexpectedCharacter(self.curPos, c))
            }
        }
    }

    fn handle_comment(&mut self, c: char) -> Option<TokenizerError> {
        match c {
            '\n' => {
                self.state = TokenizerState::Ready;
                self.advance();
                self.newline();
                None
            },

            _ => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                None
            }
        }
    }
    
    fn handle_number(&mut self, mut v: Vec<char>, p: Position, c: char) -> Option<TokenizerError> {
        match c {
            '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' | 'h' | 'H' | 'o' | 'O' => {
                v.push(c);
                self.state = TokenizerState::ReadingNumber(v, p);
                self.advance();
                None
            },
            
            ' ' | '\t' | '\r' | '\n' | ',' | '+' | '-' | '*' | '/' | ';' => {
                self.tokens.push(Token::Number(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                None
            },

            _ => {
                self.state = TokenizerState::ReadingNumber(v, p);
                Some(TokenizerError::UnexpectedCharacter(self.curPos, c))
            }
        }
    }

    fn handle_identifier(&mut self, mut v: Vec<char>, p: Position, c: char) -> Option<TokenizerError> {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => {
                v.push(c);
                self.state = TokenizerState::ReadingIdentifier(v, p);
                self.advance();
                None
            },

            _ => {
                self.tokens.push(Token::Identifier(v.into_iter().collect(), p));
                self.state = TokenizerState::Ready;
                None
            }
        }
    }

    pub fn consume_char(&mut self) -> Option<TokenizerError> {
        let c = self.current_char();
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

    pub fn tokenize(text: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut tokenizer = Tokenizer::new(text);
        loop {
            if tokenizer.eof() {
                return Ok(tokenizer.get_tokens());
            }
            if let Some(err) = tokenizer.consume_char() {
                return Err(err);
            }
        }
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

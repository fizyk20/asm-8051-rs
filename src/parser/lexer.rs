use std::mem;

#[derive(Clone, Copy)]
pub struct Position {
    row: usize,
    column: usize
}

/// An enumeration listing possible tokens
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

pub enum TokenizerError {
    GeneralError,
    UnexpectedCharacter(Position, char)
}

impl Tokenizer {

    pub fn new(s: String) -> Tokenizer {
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

    fn next_char(&self) -> Option<char> {
        if self.position + 1 < self.string.len() {
            Some(self.string[self.position + 1])
        }
        else {
            None
        }
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
                self.advance();
                None
            },

            '\n' => {
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
                self.tokens.push(Token::Colon(self.curPos));
                self.advance();
                None
            },

            ',' => {
                self.tokens.push(Token::Comma(self.curPos));
                self.advance();
                None
            },

            '@' => {
                self.tokens.push(Token::At(self.curPos));
                self.advance();
                None
            },

            '#' => {
                self.tokens.push(Token::Hash(self.curPos));
                self.advance();
                None
            },

            '+' => {
                self.tokens.push(Token::Plus(self.curPos));
                self.advance();
                None
            },

            ';' => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                None
            },

            _ => Some(TokenizerError::UnexpectedCharacter(self.curPos, c))
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
                self.advance();
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

}

use std::mem;

/// An enumeration listing possible tokens
pub enum Token {
    Identifier(String),
    Number(String),
    Colon,
    Comma,
    At,
    Hash,
    Plus,
    Newline
}

/// Possible tokenizer states
enum TokenizerState {
    Ready,
    ReadingNumber(Vec<char>),
    ReadingIdentifier(Vec<char>),
    ReadingComment,
    Invalid
}

/// The tokenizer struct
pub struct Tokenizer {
    tokens: Vec<Token>,
    string: Vec<char>,
    position: usize,
    curRow: usize,
    curCol: usize,
    state: TokenizerState
}

pub enum TokenizerError {
    GeneralError,
    UnexpectedCharacter(char)
}

impl Tokenizer {

    pub fn new(s: String) -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            string: s.chars().collect(),
            position: 0,
            curRow: 1,
            curCol: 1,
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
        self.curCol += 1;
        self.position += 1;
    }

    fn newline(&mut self) {
        self.curCol = 1;
        self.curRow += 1;
    }

    fn handle_ready(&mut self, c: char) -> Option<TokenizerError> {
        match c {
            ' ' | '\t' | '\r' => {
                self.advance();
                None
            },

            '\n' => {
                self.tokens.push(Token::Newline);
                self.advance();
                self.newline();
                None
            },

            '0' ... '9' => {
                self.state = TokenizerState::ReadingNumber(vec![c]);
                self.advance();
                None
            },

            '_' | 'a' ... 'z' | 'A' ... 'Z' => {
                self.state = TokenizerState::ReadingIdentifier(vec![c]);
                self.advance();
                None
            },

            ':' => {
                self.tokens.push(Token::Colon);
                self.advance();
                None
            },

            ',' => {
                self.tokens.push(Token::Comma);
                self.advance();
                None
            },

            '@' => {
                self.tokens.push(Token::At);
                self.advance();
                None
            },

            '#' => {
                self.tokens.push(Token::Hash);
                self.advance();
                None
            },

            '+' => {
                self.tokens.push(Token::Plus);
                self.advance();
                None
            },

            ';' => {
                self.state = TokenizerState::ReadingComment;
                self.advance();
                None
            },

            _ => Some(TokenizerError::UnexpectedCharacter(c))
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
    
    fn handle_number(&mut self, mut v: Vec<char>, c: char) -> Option<TokenizerError> {
        match c {
            '0' ... '9' | 'a' ... 'f' | 'A' ... 'F' | 'h' | 'H' | 'o' | 'O' => {
                v.push(c);
                self.state = TokenizerState::ReadingNumber(v);
                self.advance();
                None
            },
            
            ' ' | '\t' | '\r' | '\n' | ',' | '+' | '-' | '*' | '/' | ';' => {
                self.tokens.push(Token::Number(v.into_iter().collect()));
                self.state = TokenizerState::Ready;
                None
            },

            _ => {
                self.state = TokenizerState::ReadingNumber(v);
                Some(TokenizerError::UnexpectedCharacter(c))
            }
        }
    }

    fn handle_identifier(&mut self, mut v: Vec<char>, c: char) -> Option<TokenizerError> {
        match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' | '_' => {
                v.push(c);
                self.state = TokenizerState::ReadingIdentifier(v);
                self.advance();
                None
            },

            _ => {
                self.tokens.push(Token::Identifier(v.into_iter().collect()));
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
            
            TokenizerState::ReadingNumber(v) => {
                self.handle_number(v, c)
            },

            TokenizerState::ReadingIdentifier(v) => {
                self.handle_identifier(v, c)
            },

            TokenizerState::ReadingComment => {
                self.handle_comment(c)
            },

            TokenizerState::Invalid => panic!("Tokenizer caught in invalid state")
        }
    }

}

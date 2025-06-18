#[cfg(test)]
mod tests;

use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new(token::EQ, format!("{}{}", ch, self.ch));
                } else {
                    tok = token::Token::new(token::ASSIGN, format!("{}", self.ch));
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new(token::NOT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    tok = token::Token::new(token::BANG, format!("{}", self.ch));
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new(token::LT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    tok = token::Token::new(token::LT, format!("{}", self.ch));
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    tok = token::Token::new(token::GT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    tok = token::Token::new(token::GT, format!("{}", self.ch));
                }
            }
            '+' => tok = token::Token::new(token::PLUS, format!("{}", self.ch)),
            '-' => tok = token::Token::new(token::MINUS, format!("{}", self.ch)),
            '/' => tok = token::Token::new(token::SLASH, format!("{}", self.ch)),
            '*' => tok = token::Token::new(token::ASTERISK, format!("{}", self.ch)),
            ';' => tok = token::Token::new(token::SEMICOLON, format!("{}", self.ch)),
            ':' => tok = token::Token::new(token::COLON, format!("{}", self.ch)),
            ',' => tok = token::Token::new(token::COMMA, format!("{}", self.ch)),
            '"' => {
                let t = token::STRING;
                let literal = self.read_string();
                tok = token::Token::new(t, literal);
            }
            '(' => tok = token::Token::new(token::LPAREN, format!("{}", self.ch)),
            ')' => tok = token::Token::new(token::RPAREN, format!("{}", self.ch)),
            '[' => tok = token::Token::new(token::LBRACKET, format!("{}", self.ch)),
            ']' => tok = token::Token::new(token::RBRACKET, format!("{}", self.ch)),
            '{' => tok = token::Token::new(token::LBRACE, format!("{}", self.ch)),
            '}' => tok = token::Token::new(token::RBRACE, format!("{}", self.ch)),
            '\0' => tok = token::Token::new(token::EOF, format!("")),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    let lookup = literal.clone();
                    let t = token::lookup_ident(lookup);

                    tok = token::Token::new(t, literal);
                    return tok;
                } else if self.ch.is_ascii_digit() {
                    let t = token::INT;
                    let literal = self.read_number();

                    tok = token::Token::new(t, literal);
                    return tok;
                } else {
                    tok = token::Token::new(token::ILLEGAL, format!("{}", self.ch));
                }
            }
        }

        self.read_char();
        return tok;
    }

    fn peek_char(&self) -> char {
        let mut temp = self.input.chars();

        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return temp.nth(self.read_position).unwrap();
        }
    }

    fn read_char(&mut self) {
        let mut temp = self.input.chars();

        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = temp.nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char()
        }

        return self.input[position..self.position].to_string();
    }

    fn read_string(&mut self) -> String {
        self.read_char();
        let position = self.position;

        while self.ch != '"' {
            self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_alphabetic() && self.ch != ';' {
            self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

pub fn new(input: String) -> Lexer {
    let ch = input.clone().chars().nth(0).unwrap();
    let mut ret = Lexer {
        input,
        position: 0,
        read_position: 0,
        ch,
    };

    ret.read_char();
    return ret;
}

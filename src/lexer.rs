use crate::token;

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> token::Token {
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    return token::Token::new(token::EQ, format!("{}{}", ch, self.ch));
                } else {
                    return token::Token::new(token::ASSIGN, format!("{}", self.ch));
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    return token::Token::new(token::NOT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    return token::Token::new(token::BANG, format!("{}", self.ch));
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    return token::Token::new(token::LT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    return token::Token::new(token::LT, format!("{}", self.ch));
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    return token::Token::new(token::GT_EQ, format!("{}{}", ch, self.ch));
                } else {
                    return token::Token::new(token::GT, format!("{}", self.ch));
                }
            }
            '+' => return token::Token::new(token::PLUS, format!("{}", self.ch)),
            '-' => return token::Token::new(token::MINUS, format!("{}", self.ch)),
            '/' => return token::Token::new(token::SLASH, format!("{}", self.ch)),
            '*' => return token::Token::new(token::ASTERISK, format!("{}", self.ch)),
            ';' => return token::Token::new(token::SEMICOLON, format!("{}", self.ch)),
            ':' => return token::Token::new(token::COLON, format!("{}", self.ch)),
            ',' => return token::Token::new(token::COMMA, format!("{}", self.ch)),
            '"' => return token::Token::new(token::STRING, format!("{}", self.ch)),
            '(' => return token::Token::new(token::LPAREN, format!("{}", self.ch)),
            ')' => return token::Token::new(token::RPAREN, format!("{}", self.ch)),
            '[' => return token::Token::new(token::LBRACKET, format!("{}", self.ch)),
            ']' => return token::Token::new(token::RBRACKET, format!("{}", self.ch)),
            '{' => return token::Token::new(token::LBRACE, format!("{}", self.ch)),
            '}' => return token::Token::new(token::RBRACE, format!("{}", self.ch)),
            '\0' => return token::Token::new(token::EOF, format!("")),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let literal = self.read_identifier();
                    let lookup = literal.clone();
                    let t = token::lookup_ident(lookup);
                    return token::Token::new(t, literal);
                } else if self.ch.is_ascii_digit() {
                    let t = token::INT;
                    let literal = self.read_number();
                    return token::Token::new(t, literal);
                } else {
                    return token::Token::new(token::ILLEGAL, format!("{}", self.ch));
                }
            }
        }
    }

    fn peek_char(&self) -> char {
        let mut temp = self.input.chars();

        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return temp.nth(self.position).unwrap();
        }
    }

    fn read_char(&mut self) {
        let mut temp = self.input.chars();
        if self.read_position >= self.input.len() {
            self.ch = '0';
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

        return self.input[position..self.read_position].to_string();
    }

    fn read_string(&mut self) -> String {
        let position = self.position;

        while self.ch.is_alphabetic() {
            self.read_char();
        }
        return self.input[position..self.read_position].to_string();
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }

        return self.input[position..self.read_position].to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_test() {
        assert!(1 == 3);
    }
}

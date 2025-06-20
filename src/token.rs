use std::collections::HashMap;

pub type TokenType = &'static str;

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type,
            literal,
        };
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

pub const ASSIGN: &str = "ASSIGN";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const SLASH: &str = "/";
pub const ASTERISK: &str = "*";
pub const BANG: &str = "!";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "=";
pub const NOT_EQ: &str = "!=";
pub const LT_EQ: &str = "<=";
pub const GT_EQ: &str = ">=";

pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACKET: &str = "[";
pub const RBRACKET: &str = "]";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "return";
pub const STRING: &str = "STRING";

const KEYWORDS: [(&str, TokenType); 7] = [
    ("fn", FUNCTION),
    ("let", LET),
    ("true", TRUE),
    ("false", FALSE),
    ("if", IF),
    ("else", ELSE),
    ("return", RETURN),
];

pub fn lookup_ident(ident: String) -> TokenType {
    for (k, v) in KEYWORDS.iter() {
        if k == &ident {
            return v;
        }
    }

    IDENT
}

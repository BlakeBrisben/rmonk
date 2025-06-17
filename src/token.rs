use std::collections::HashMap;

pub type TokenType = &'static str;

pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

impl Token {
    pub fn new(Type: TokenType, Literal: String) -> Token {
        return Token { Type, Literal };
    }
}

// TODO: I think making TokenType into an enum like this is ok, but I need to go through the code
// again to make sure that it is
enum TYPES {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    SLASH,
    ASTERISK,
    BANG,
    LT,
    GT,
    EQ,
    NOT_EQ,
    LT_EQ,
    GT_EQ,

    COMMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    STRING,
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

pub fn lookup_ident(ident: String) -> TokenType {
    let keywords: HashMap<String, TokenType> = HashMap::from([
        (String::from("fn"), FUNCTION),
        (String::from("let"), LET),
        (String::from("true"), TRUE),
        (String::from("false"), FALSE),
        (String::from("if"), IF),
        (String::from("else"), ELSE),
        (String::from("return"), RETURN),
    ]);

    match keywords.get(ident.as_str()) {
        Some(t) => t,
        None => IDENT,
    }
}

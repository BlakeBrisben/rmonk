use super::*;
use crate::token;

#[test]
fn test_string() {
    let program = Program {
        statements: vec![
            Box::new(LetStatement {
                token: token::Token::new(token::LET, String::from("let")),
                name: Box::new(Identifier {
                    token: token::Token::new(token::IDENT, String::from("myVar")),
                    value: String::from("myVar"),
                }),
                value: Box::new(Identifier {
                    token: token::Token::new(token::IDENT, String::from("anotherVar")),
                    value: String::from("anotherVar"),
                }),
            }),
            Box::new(LetStatement {
                token: token::Token::new(token::LET, String::from("let")),
                name: Box::new(Identifier {
                    token: token::Token::new(token::IDENT, String::from("myVar")),
                    value: String::from("myVar"),
                }),
                value: Box::new(Identifier {
                    token: token::Token::new(token::IDENT, String::from("anotherVar")),
                    value: String::from("anotherVar"),
                }),
            }),
        ],
    };

    assert_eq!(
        program.string(),
        String::from("let myVar = anotherVar;\nlet myVar = anotherVar;\n")
    );
}

#[test]
fn test_return() {
    let program = Program {
        statements: vec![Box::new(ReturnStatement {
            token: token::Token::new(token::RETURN, String::from("return")),
            value: Box::new(IntegerLiteral {
                token: token::Token::new(token::INT, String::from("5")),
                value: 5,
            }),
        })],
    };

    assert_eq!(program.string(), String::from("return 5;\n"));
}

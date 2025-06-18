use super::*;

#[test]
fn test_next_token() {
    let input = String::from(
        "
            let five = 5;
            x + y;
            fn(x, y) {
                x + y;
            };
            !-/*<>;
            (5 + 5);
            if ( 5 < 10) {
                return true;
            } else {
                return false;
            }
            5 == 5;
            5 != 5;
            5 >= 5;
            5 <= 5;
            \"foobar\";
            \"foo bar\";
            [5, 5];
            {\"foo\": \"bar\"};
        ",
    );

    struct Test {
        expected_type: token::TokenType,
        expected_literal: String,
    }

    let tests = vec![
        Test {
            expected_type: token::LET,
            expected_literal: String::from("let"),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("five"),
        },
        Test {
            expected_type: token::ASSIGN,
            expected_literal: String::from("="),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("x"),
        },
        Test {
            expected_type: token::PLUS,
            expected_literal: String::from("+"),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("y"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::FUNCTION,
            expected_literal: String::from("fn"),
        },
        Test {
            expected_type: token::LPAREN,
            expected_literal: String::from("("),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("x"),
        },
        Test {
            expected_type: token::COMMA,
            expected_literal: String::from(","),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("y"),
        },
        Test {
            expected_type: token::RPAREN,
            expected_literal: String::from(")"),
        },
        Test {
            expected_type: token::LBRACE,
            expected_literal: String::from("{"),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("x"),
        },
        Test {
            expected_type: token::PLUS,
            expected_literal: String::from("+"),
        },
        Test {
            expected_type: token::IDENT,
            expected_literal: String::from("y"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::RBRACE,
            expected_literal: String::from("}"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::BANG,
            expected_literal: String::from("!"),
        },
        Test {
            expected_type: token::MINUS,
            expected_literal: String::from("-"),
        },
        Test {
            expected_type: token::SLASH,
            expected_literal: String::from("/"),
        },
        Test {
            expected_type: token::ASTERISK,
            expected_literal: String::from("*"),
        },
        Test {
            expected_type: token::LT,
            expected_literal: String::from("<"),
        },
        Test {
            expected_type: token::GT,
            expected_literal: String::from(">"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::LPAREN,
            expected_literal: String::from("("),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::PLUS,
            expected_literal: String::from("+"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::RPAREN,
            expected_literal: String::from(")"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::IF,
            expected_literal: String::from("if"),
        },
        Test {
            expected_type: token::LPAREN,
            expected_literal: String::from("("),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::LT,
            expected_literal: String::from("<"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("10"),
        },
        Test {
            expected_type: token::RPAREN,
            expected_literal: String::from(")"),
        },
        Test {
            expected_type: token::LBRACE,
            expected_literal: String::from("{"),
        },
        Test {
            expected_type: token::RETURN,
            expected_literal: String::from("return"),
        },
        Test {
            expected_type: token::TRUE,
            expected_literal: String::from("true"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::RBRACE,
            expected_literal: String::from("}"),
        },
        Test {
            expected_type: token::ELSE,
            expected_literal: String::from("else"),
        },
        Test {
            expected_type: token::LBRACE,
            expected_literal: String::from("{"),
        },
        Test {
            expected_type: token::RETURN,
            expected_literal: String::from("return"),
        },
        Test {
            expected_type: token::FALSE,
            expected_literal: String::from("false"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::RBRACE,
            expected_literal: String::from("}"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::EQ,
            expected_literal: String::from("=="),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::NOT_EQ,
            expected_literal: String::from("!="),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::GT_EQ,
            expected_literal: String::from(">="),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::LT_EQ,
            expected_literal: String::from("<="),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::STRING,
            expected_literal: String::from("foobar"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::STRING,
            expected_literal: String::from("foo bar"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::LBRACKET,
            expected_literal: String::from("["),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::COMMA,
            expected_literal: String::from(","),
        },
        Test {
            expected_type: token::INT,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: token::RBRACKET,
            expected_literal: String::from("]"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: token::LBRACE,
            expected_literal: String::from("{"),
        },
        Test {
            expected_type: token::STRING,
            expected_literal: String::from("foo"),
        },
        Test {
            expected_type: token::COLON,
            expected_literal: String::from(":"),
        },
        Test {
            expected_type: token::STRING,
            expected_literal: String::from("bar"),
        },
        Test {
            expected_type: token::RBRACE,
            expected_literal: String::from("}"),
        },
        Test {
            expected_type: token::SEMICOLON,
            expected_literal: String::from(";"),
        },
    ];

    let mut l = new(input);

    for test in tests.iter() {
        let tok = l.next_token();

        println!(
            "TYPE:(found, expected) {}: {}",
            tok.token_type, test.expected_type
        );
        println!(
            "LITERAL:(found, expected) {}: {}\n",
            tok.literal, test.expected_literal
        );

        assert_eq!(tok.token_type, test.expected_type);
        assert_eq!(tok.literal, test.expected_literal);
    }
}

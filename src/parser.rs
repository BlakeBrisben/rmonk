use crate::ast;
use crate::lexer;
use crate::token;
use std::collections::HashMap;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;
const INDEX: u8 = 8;

const PRECEDENCES: [(token::TokenType, u8); 10] = [
    (token::EQ, EQUALS),
    (token::NOT_EQ, EQUALS),
    (token::LT, EQUALS),
    (token::GT, EQUALS),
    (token::PLUS, EQUALS),
    (token::MINUS, EQUALS),
    (token::SLASH, EQUALS),
    (token::ASTERISK, EQUALS),
    (token::LPAREN, EQUALS),
    (token::RPAREN, EQUALS),
];

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn ast::Expression>>;
type InfixParseFn = fn(&mut Parser, &Box<dyn ast::Expression>) -> Option<Box<dyn ast::Expression>>;

pub struct Parser {
    l: Box<lexer::Lexer>,
    errors: Vec<String>,

    cur_token: Box<token::Token>,
    peek_token: Box<token::Token>,

    prefix_parse_fns: HashMap<token::TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<token::TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut l: Box<lexer::Lexer>) -> Parser {
        let tok = l.next_token();

        let mut p = Parser {
            l,
            errors: Vec::new(),
            cur_token: Box::new(token::Token::new(token::ILLEGAL, String::from(""))),
            peek_token: Box::new(token::Token::new(tok.token_type, tok.literal)),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        p.next_token();
        p.next_token();

        // TODO: Need to figure out how to give multiple possible options for the parameter (option
        // version and non-option version)
        p.register_prefix_fns(token::IDENT, Parser::parse_identifier);
        p.register_prefix_fns(token::INT, Parser::parse_integer_literal);
        p.register_prefix_fns(token::INT, Parser::parse_function_literal);
        p.register_prefix_fns(token::INT, Parser::parse_string_literal);
        p.register_prefix_fns(token::INT, Parser::parse_array_literal);

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = Box::new(self.l.next_token());
    }

    fn register_prefix_fns(&mut self, tok: token::TokenType, f: PrefixParseFn) {
        self.prefix_parse_fns.insert(tok, f);
    }

    fn register_infix_fns(&mut self, tok: token::TokenType, f: InfixParseFn) {
        self.infix_parse_fns.insert(tok, f);
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::Identifier {
            token: *self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::IntegerLiteral {
            token: *self.cur_token.clone(),
            value: self
                .cur_token
                .literal
                .parse::<i64>()
                .expect("Integer literal was not an integer"),
        }))
    }

    fn parse_string_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::StringLiteral {
            token: *self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::ArrayLiteral {
            token: *self.cur_token.clone(),
            elements: self
                .parse_expression_list(token::RBRACKET)
                .expect("Unable to parse expression list"),
        }))
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        let token = *self.cur_token.clone();

        if !self.expect_peek(token::LPAREN) {
            return None;
        }

        let parameters = self
            .parse_function_parameters()
            .expect("Unable to parse function parameters");

        if !self.expect_peek(token::LBRACE) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Box::new(ast::FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    // TODO: Need to figure out how to impl PartialEq and Eq to make this work
    // fn parse_hash_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
    //     let token = *self.cur_token.clone();
    //
    //     let mut pairs: HashMap<
    //         Box<dyn ast::Expression + 'static>,
    //         Box<dyn ast::Expression + 'static>,
    //     > = HashMap::new();
    //
    //     while !self.peek_token_is(token::RBRACE) {
    //         self.next_token();
    //
    //         let key = self
    //             .parse_expression(LOWEST)
    //             .expect("Unable to parse Hash key");
    //
    //         if !self.expect_peek(token::COLON) {
    //             return None;
    //         }
    //
    //         self.next_token();
    //         let value = self
    //             .parse_expression(LOWEST)
    //             .expect("Unable to parse Hash value");
    //
    //         pairs.insert(key, value);
    //     }
    //
    //     None
    // }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = *self.cur_token.clone();

        if !self.expect_peek(token::IDENT) {
            return None;
        }

        let name = Box::new(ast::Identifier {
            token,
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(token::ASSIGN) {
            return None;
        }

        self.next_token();

        let value = self
            .parse_expression(LOWEST)
            .expect("Unable to parse expression");

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ast::LetStatement {
            token: *self.cur_token.clone(),
            name,
            value,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let token = *self.cur_token.clone();

        self.next_token();

        let value = self
            .parse_expression(LOWEST)
            .expect("Unable to parse expression");

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(ast::ReturnStatement { token, value }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let expression = self
            .parse_expression(LOWEST)
            .expect("Unable to parse expression");

        let stmt: Option<Box<dyn ast::Statement>> = Some(Box::new(ast::ExpressionStatement {
            token: *self.cur_token.clone(),
            expression,
        }));

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        stmt
    }

    fn parse_expression(&mut self, prec: u8) -> Option<Box<dyn ast::Expression>> {
        let prefix = self.prefix_parse_fns.get(self.cur_token.token_type);

        match prefix {
            Some(prefix) => {
                let mut left_exp = prefix(self);
                let mut left: Box<dyn ast::Expression>;

                match left_exp {
                    Some(l) => left = l,
                    None => {
                        return None;
                    }
                }

                while !self.peek_token_is(token::SEMICOLON) && prec < self.peek_precedence() {
                    let infix_fns = self.infix_parse_fns.clone();
                    let infix = infix_fns.get(self.peek_token.token_type);

                    match infix {
                        Some(infix) => {
                            self.next_token();
                            left_exp = infix(self, &left);
                            match left_exp {
                                Some(l) => left = l,
                                None => {
                                    self.new_error("Unable to parse using infix operator");
                                }
                            }
                        }
                        None => {
                            return Some(left);
                        }
                    }
                }

                return Some(left);
            }
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.token_type);
                return None;
            }
        }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            token::LET => return self.parse_let_statement(),
            token::RETURN => return self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
        }
    }

    fn parse_expression_list(
        &mut self,
        end: token::TokenType,
    ) -> Option<Vec<Box<dyn ast::Expression>>> {
        let mut list = Vec::new();

        if self.peek_token_is(end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(
            self.parse_expression(LOWEST)
                .expect("Unable to parse expression"),
        );

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            list.push(
                self.parse_expression(LOWEST)
                    .expect("Unable to parse expression"),
            );
        }

        if !self.expect_peek(end) {
            self.new_error(format!("Did not find {} at end of expression list", end).as_str());
            return None;
        }
        Some(list)
    }

    fn parse_block_statement(&mut self) -> Box<ast::BlockStatement> {
        let mut stmts = Vec::new();

        let token = *self.cur_token.clone();

        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(stmt) => stmts.push(stmt),
                None => (),
            }

            self.next_token();
        }

        Box::new(ast::BlockStatement {
            token,
            statements: stmts,
        })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Box<ast::Identifier>>> {
        let mut idents = Vec::new();

        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return Some(idents);
        }

        self.next_token();

        let ident = Box::new(ast::Identifier {
            token: *self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        idents.push(ident);

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();

            let ident = Box::new(ast::Identifier {
                token: *self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
            idents.push(ident);
        }

        if !self.expect_peek(token::RPAREN) {
            return None;
        }

        Some(idents)
    }

    fn cur_token_is(&self, tt: token::TokenType) -> bool {
        self.cur_token.token_type == tt
    }

    fn peek_token_is(&self, tt: token::TokenType) -> bool {
        self.peek_token.token_type == tt
    }

    fn expect_peek(&mut self, tt: token::TokenType) -> bool {
        if self.peek_token_is(tt) {
            self.next_token();
            return true;
        } else {
            self.peek_error(tt);
            return false;
        }
    }

    fn peek_precedence(&self) -> u8 {
        for prec in PRECEDENCES.iter() {
            if prec.0 == self.peek_token.token_type {
                return prec.1;
            }
        }
        return LOWEST;
    }

    fn new_error(&mut self, msg: &str) {
        self.errors.push(String::from(msg));
    }

    fn no_prefix_parse_fn_error(&mut self, tt: token::TokenType) {
        self.errors.push(String::from(format!(
            "no prefix parse function for {} found",
            tt
        )));
    }

    fn peek_error(&mut self, tt: token::TokenType) {
        self.errors.push(String::from(format!(
            "Expected token to be {}, got {} instead",
            tt, self.peek_token.token_type
        )));
    }
}

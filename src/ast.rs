use crate::token;

pub trait Statement {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn statement_node(&self);
}

pub trait Expression {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return String::from("");
        }
    }
    fn string(&self) -> String {
        let mut ret = String::new();

        for stmt in self.statements.iter() {
            ret = String::from(format!("{}\n{}", ret, stmt.string()));
        }

        ret
    }
    fn statement_node(&self) {}
}

pub struct Identifier {
    token: token::Token,
    value: String,
}

impl Expression for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn expression_node(&self) {}
}

// STATEMENTS

pub struct LetStatement<'a> {
    token: token::Token,
    name: &'a Identifier,
    value: Box<dyn Expression>,
}

impl<'a> Statement for LetStatement<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let ret = String::from(format!(
            "{} {} = {};",
            self.token_literal(),
            self.name.string(),
            self.value.string(),
        ));

        ret
    }

    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    token: token::Token,
    value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let ret = String::from(format!("{} {};", self.token_literal(), self.value.string()));

        ret
    }

    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    token: token::Token,
    expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.expression.string()
    }

    fn statement_node(&self) {}
}

pub struct BlockStatement {
    token: token::Token,
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        for stmt in self.statements.iter() {
            ret.push_str(stmt.string().as_str());
        }
        ret
    }

    fn statement_node(&self) {}
}

// EXPRESSIONS

pub struct IntegerLiteral {
    token: token::Token,
    value: i64,
}

impl Expression for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn expression_node(&self) {}
}

pub struct Boolean {
    token: token::Token,
    value: bool,
}

impl Expression for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn expression_node(&self) {}
}

pub struct IfExpression<'a> {
    token: token::Token,
    condition: Box<dyn Expression>,
    consequence: &'a BlockStatement,
    alternative: Option<&'a BlockStatement>,
}

impl<'a> Expression for IfExpression<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::from(format!(
            "if {} {}",
            self.condition.string(),
            self.consequence.string()
        ));

        match self.alternative {
            Some(block) => ret.push_str(format!("else {}", block.string()).as_str()),
            None => return ret,
        }

        ret
    }

    fn expression_node(&self) {}
}


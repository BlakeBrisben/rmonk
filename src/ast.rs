use crate::token;
use std::collections::HashMap;

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
            ret = String::from(format!("{}{}\n", ret, stmt.string()));
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

pub struct LetStatement {
    token: token::Token,
    name: Box<Identifier>,
    value: Box<dyn Expression>,
}

impl Statement for LetStatement {
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

pub struct StringLiteral {
    token: token::Token,
    value: String,
}

impl Expression for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }

    fn expression_node(&self) {}
}

pub struct ArrayLiteral {
    token: token::Token,
    elements: Vec<Box<dyn Expression>>,
}

impl Expression for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str("[");

        for elem in self.elements.iter() {
            ret.push_str(format!("{}, ", elem.string()).as_str());
        }

        ret.push_str("]");

        ret
    }

    fn expression_node(&self) {}
}

pub struct FunctionLiteral {
    token: token::Token,
    parameters: Vec<Box<Identifier>>,
    body: Box<BlockStatement>,
}

impl Expression for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str(format!("{}(", self.token_literal()).as_str());

        for param in self.parameters.iter() {
            ret.push_str(format!("{}, ", param.string()).as_str());
        }

        ret.push_str(format!(") {}", self.body.string()).as_str());

        ret
    }

    fn expression_node(&self) {}
}

pub struct HashLiteral {
    token: token::Token,
    pairs: HashMap<Box<dyn Expression>, Box<dyn Expression>>,
}

impl Expression for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str("{");
        for (k, v) in self.pairs.iter() {
            ret.push_str(format!("{}: {}, ", k.string(), v.string()).as_str());
        }
        ret.push_str("{");

        ret
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

pub struct IfExpression {
    token: token::Token,
    condition: Box<dyn Expression>,
    consequence: Box<BlockStatement>,
    alternative: Option<Box<BlockStatement>>,
}

impl Expression for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::from(format!(
            "if {} {}",
            self.condition.string(),
            self.consequence.string()
        ));

        match &self.alternative {
            Some(block) => ret.push_str(format!("else {}", block.string()).as_str()),
            None => return ret,
        }

        ret
    }

    fn expression_node(&self) {}
}

pub struct CallExpression {
    token: token::Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str(format!("{}(", self.function.string()).as_str());

        for arg in self.arguments.iter() {
            ret.push_str(format!("{} ,", arg.string()).as_str());
        }

        ret.push_str(")");

        ret
    }

    fn expression_node(&self) {}
}

pub struct PrefixExpression {
    token: token::Token,
    operator: String,
    right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str(format!("({}{})", self.operator, self.right.string()).as_str());

        ret
    }

    fn expression_node(&self) {}
}

pub struct InfixExpression {
    token: token::Token,
    operator: String,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str(
            format!(
                "({} {} {})",
                self.left.string(),
                self.operator,
                self.right.string()
            )
            .as_str(),
        );

        ret
    }

    fn expression_node(&self) {}
}

pub struct IndexExpression {
    token: token::Token,
    left: Box<dyn Expression>,
    index: Box<dyn Expression>,
}

impl Expression for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut ret = String::new();

        ret.push_str(format!("{}[{}]", self.left.string(), self.index.string()).as_str());

        ret
    }

    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests;

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
    pub statements: Vec<Box<dyn Statement>>,
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

// STATEMENTS

pub struct LetStatement {
    pub token: token::Token,
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
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
    pub token: token::Token,
    pub value: Box<dyn Expression>,
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
    pub token: token::Token,
    pub expression: Box<dyn Expression>,
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
    pub token: token::Token,
    pub statements: Vec<Box<dyn Statement>>,
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

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
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

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
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
    pub token: token::Token,
    pub value: String,
}

impl Expression for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let ret = String::from(format!("\"{}\"", self.token.literal.clone()));
        ret
    }

    fn expression_node(&self) {}
}

pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<Box<dyn Expression>>,
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
    pub token: token::Token,
    pub parameters: Vec<Box<Identifier>>,
    pub body: Box<BlockStatement>,
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
    pub token: token::Token,
    pub pairs: HashMap<Box<dyn Expression>, Box<dyn Expression>>,
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
    pub token: token::Token,
    pub value: bool,
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
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: Box<BlockStatement>,
    pub alternative: Option<Box<BlockStatement>>,
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
    pub token: token::Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
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
    pub token: token::Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
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
    pub token: token::Token,
    pub operator: String,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
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
    pub token: token::Token,
    pub left: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
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

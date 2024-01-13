use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node() {}
}
pub trait Expression: Node {
    fn expression_node() {}
}

// Root node
pub struct Program {
    statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            return "";
        }
        self.statements[0].token_literal()
    }
}

// Statement nodes
pub struct LetStatement<T: Expression> {
    token: Token,
    name: Identifier,
    value: T,
}

impl<T: Expression> Statement for LetStatement<T> {}
impl<T: Expression> Node for LetStatement<T> {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

pub struct Identifier {
    token: Token,
    value: String,
}

impl Expression for Identifier {}
impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal()
    }
}

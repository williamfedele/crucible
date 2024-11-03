#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Comparison {
        op: ComparisonOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        typ: Type,
        value: Expr,
    },
    Assignment {
        target: String,
        value: Expr,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
}

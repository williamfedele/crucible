#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Boolean(bool),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Comparison {
        op: ComparisonOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Equal,
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
    And,
    Or,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    Void,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expr),
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
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
}

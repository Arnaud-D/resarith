#[derive(Debug, PartialEq)]
pub enum Operator {
    Series,
    Parallel,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Value(f64),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(f64),
    Operation {
        operator: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

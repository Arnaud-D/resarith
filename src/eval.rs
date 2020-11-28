use crate::defs::{Expr, Operator};

pub fn eval(expr: Expr) -> f64 {
    match expr {
        Expr::Value(v) => v,
        Expr::Operation { operator, left, right } => {
            let l = eval(*left);
            let r = eval(*right);
            match operator {
                Operator::Series => l + r,
                Operator::Parallel => l * r / (l + r),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_1() {
        let parallels = Expr::Operation {
            operator: Operator::Parallel,
            left: Box::new(Expr::Value(10.0)),
            right: Box::new(Expr::Value(10.0)),
        };
        let expr = Expr::Operation {
            operator: Operator::Series,
            left: Box::new(parallels),
            right: Box::new(Expr::Value(7.0)),
        };
        let result = eval(expr);
        assert_eq!(result, 12.0);
    }
}

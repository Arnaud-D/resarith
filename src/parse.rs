use crate::defs::{Expr, Token};

pub fn parse(tokens: &mut Vec<Token>) -> Expr {
    tokens.reverse();
    parse_expr(tokens)
}

// Expr -> Expr1 Expr2
fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    let expr = parse_expr1(tokens);
    parse_expr2(tokens, expr)
}

// Expr1 ->  value | '(' Expr ')'
fn parse_expr1(tokens: &mut Vec<Token>) -> Expr {
    match tokens.pop() {
        Some(Token::Value(v)) => {
            Expr::Value(v)
        }
        Some(Token::LeftParenthesis) => {
            let expr = parse_expr(tokens);
            match tokens.pop() {
                Some(Token::RightParenthesis) => expr,
                _ => panic!("Syntax error: expected ')', got something else.")
            }
        }
        _ => panic!("Syntax error: expected value or '(', got something else.")
    }
}

// Expr2 -> ':' Expr1 | '|' Expr1 | eps
fn parse_expr2(tokens: &mut Vec<Token>, expr: Expr) -> Expr {
    match tokens.pop() {
        Some(Token::Operator(op)) => {
            let right = parse_expr1(tokens);
            Expr::Operation {
                operator: op,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }
        Some(t) => {
            tokens.push(t);
            expr
        }
        None => expr,
    }
}


#[cfg(test)]
mod tests {
    use crate::defs::Operator;

    use super::*;

    #[test]
    fn test_parse_1() {
        let mut tokens = vec!(
            Token::LeftParenthesis,
            Token::Value(7.0),
            Token::Operator(Operator::Series),
            Token::LeftParenthesis,
            Token::LeftParenthesis,
            Token::Value(5.0),
            Token::Operator(Operator::Parallel),
            Token::Value(4.0),
            Token::RightParenthesis,
            Token::RightParenthesis,
            Token::RightParenthesis
        );

        let expr = Expr::Operation {
            operator: Operator::Series,
            left: Box::new(Expr::Value(7.0)),
            right: Box::new(
                Expr::Operation {
                    operator: Operator::Parallel,
                    left: Box::new(Expr::Value(5.0)),
                    right: Box::new(Expr::Value(4.0)),
                }
            ),
        };

        let result = parse(&mut tokens);

        assert_eq!(expr, result);
    }
}

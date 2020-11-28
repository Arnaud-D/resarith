use crate::defs::{Operator, Token};

pub fn tokenize(string: String) -> Vec<Token> {
    let mut chars = string.chars().collect::<Vec<char>>();
    chars.reverse();
    let mut tokens = Vec::new();
    while chars.len() > 0 {
        match read_token(&mut chars) {
            Some(t) => tokens.push(t),
            None => panic!("Error: Unexpected character during parsing.")
        }
    }
    tokens
}

pub enum State {
    Init,
    SkipWhiteSpace,
    OperatorParenthesis,
    NumberIntegral,
    NumberDecimal,
    NumberFull,
    Fail,
}

fn read_token(chars: &mut Vec<char>) -> Option<Token> {
    let mut state = State::Init;
    let mut buffer = String::new();
    loop {
        match state {
            State::Init => {
                match chars.pop().unwrap() {
                    ' ' => state = State::SkipWhiteSpace,
                    c => {
                        buffer.push(c);
                        match c {
                            '(' | ')' | ':' | '|' => state = State::OperatorParenthesis,
                            '0'..='9' => state = State::NumberIntegral,
                            _ => state = State::Fail,
                        }
                    }
                }
            }
            State::SkipWhiteSpace => {
                match chars.pop().unwrap() {
                    ' ' => state = State::SkipWhiteSpace,
                    c => {
                        buffer.push(c);
                        match c {
                            '(' | ')' | ':' | '|' => state = State::OperatorParenthesis,
                            '0'..='9' => state = State::NumberIntegral,
                            _ => state = State::Fail,
                        }
                    }
                }
            }
            State::OperatorParenthesis => {
                match &*buffer {
                    "(" => return Some(Token::LeftParenthesis),
                    ")" => return Some(Token::RightParenthesis),
                    ":" => return Some(Token::Operator(Operator::Series)),
                    "|" => return Some(Token::Operator(Operator::Parallel)),
                    _ => panic!("The programmer is a fool.")
                }
            }
            State::NumberIntegral => {
                match chars.pop() {
                    c @ Some('0'..='9') => {
                        buffer.push(c.unwrap());
                        state = State::NumberIntegral;
                    }
                    c @ Some('.') => {
                        buffer.push(c.unwrap());
                        state = State::NumberDecimal;
                    }
                    Some(c) => {
                        chars.push(c);
                        state = State::NumberFull;
                    }
                    None => state = State::NumberFull
                }
            }
            State::NumberDecimal => {
                match chars.pop() {
                    c @ Some('0'..='9') => {
                        buffer.push(c.unwrap());
                        state = State::NumberDecimal;
                    }
                    Some(c) => {
                        chars.push(c);
                        state = State::NumberFull
                    }
                    None => state = State::NumberFull
                }
            }
            State::NumberFull => {
                let v = buffer.parse::<f64>().unwrap();
                return Some(Token::Value(v));
            }
            State::Fail => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_1() {
        let string = String::from("(3    | ((5 : 4))   )");

        let tokens = vec!(
            Token::LeftParenthesis,
            Token::Value(3.0),
            Token::Operator(Operator::Parallel),
            Token::LeftParenthesis,
            Token::LeftParenthesis,
            Token::Value(5.0),
            Token::Operator(Operator::Series),
            Token::Value(4.0),
            Token::RightParenthesis,
            Token::RightParenthesis,
            Token::RightParenthesis,
        );

        let result = tokenize(string);

        assert_eq!(tokens, result);
    }
}

use crate::token::Token;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(u32),
}

pub fn expr<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)>,
{
    let mut node = mul(line, tokens);

    while let Some((_, token)) = tokens.peek() {
        if *token == Token::Plus {
            tokens.next();
            let lhs = node;
            let rhs = mul(line, tokens);
            node = Box::new(Node::Add(lhs, rhs));
        } else if *token == Token::Minus {
            tokens.next();
            let lhs = node;
            let rhs = mul(line, tokens);
            node = Box::new(Node::Sub(lhs, rhs));
        } else {
            break;
        }
    }
    node
}

pub fn mul<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)>,
{
    let mut node = primary(line, tokens);

    while let Some((_, token)) = tokens.peek() {
        if *token == Token::Asterisk {
            tokens.next();
            let lhs = node;
            let rhs = primary(line, tokens);
            node = Box::new(Node::Mul(lhs, rhs));
        } else if *token == Token::Slash {
            tokens.next();
            let lhs = node;
            let rhs = primary(line, tokens);
            node = Box::new(Node::Div(lhs, rhs));
        } else {
            break;
        }
    }

    node
}

pub fn primary<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)>,
{
    if let Some((_, Token::OpenParenthesis)) = tokens.peek() {
        tokens.next();
        let node = expr(line, tokens);
        if let Some((i, token)) = tokens.next() {
            if token == Token::CloseParenthesis {
                return node;
            }
            crate::show_error_panic("')' がありません", line, i);
        }
        crate::show_error_panic("')' がありません", line, line.len());
    } else if let Some((_, Token::Num(_))) = tokens.peek() {
        if let Some((_, Token::Num(num))) = tokens.next() {
            return Box::new(Node::Num(num));
        }
    }

    if let Some((i, _)) = tokens.next() {
        crate::show_error_panic("数ではありません", line, i);
    }
    crate::show_error_panic("数ではありません", line, line.len());
}

#[cfg(test)]
mod test {
    use super::Node::*;
    use super::*;
    use crate::token::tokenize;

    #[test]
    fn test_expr1() {
        let s = "1+2-3";
        let mut tokens = tokenize(s).into_iter().peekable();
        assert_eq!(
            expr(s, &mut tokens),
            Box::new(Sub(
                Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
                Box::new(Num(3))
            ))
        );
    }

    #[test]
    fn test_expr2() {
        let s = "1+2*3-4";
        let mut tokens = tokenize(s).into_iter().peekable();
        assert_eq!(
            expr(s, &mut tokens),
            Box::new(Sub(
                Box::new(Add(
                    Box::new(Num(1)),
                    Box::new(Mul(Box::new(Num(2)), Box::new(Num(3))))
                )),
                Box::new(Num(4))
            ))
        );
    }

    #[test]
    fn test_expr3() {
        let s = "1+(2+3)-4";
        let mut tokens = tokenize(s).into_iter().peekable();
        assert_eq!(
            expr(s, &mut tokens),
            Box::new(Sub(
                Box::new(Add(
                    Box::new(Num(1)),
                    Box::new(Add(Box::new(Num(2)), Box::new(Num(3))))
                )),
                Box::new(Num(4))
            ))
        );
    }
}
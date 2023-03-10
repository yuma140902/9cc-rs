//! 抽象構文木(AST)関係のモジュール
use crate::{show_error_panic, token::Token};
use std::iter::Peekable;

/// 抽象構文木
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// 二項演算
    Binary(BinOp, Box<Node>, Box<Node>),
    /// 単項演算子
    Unary(UnaryOp, Box<Node>),
    /// 数値
    Num(u32),
}

/// 二項演算の種類を表すenum
#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// 単項演算子の種類を表すenum
#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Positive,
    Negative,
}

/// トークン列からexprを読み込む
///
/// exprとして解釈可能な最長の部分を取り出して抽象構文木の部分木を作る。
/// exprとは `mul ("+" mul | "-" mul)*` である。
///
/// - line - ソースコードの行。エラー表示に用いる。
/// - tokens - (usize,
/// [`Token`])のPeekableなイテレータ。usizeはトークンのソースコード中での位置である
pub fn expr<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)> + std::fmt::Debug,
{
    let mut node = mul(line, tokens);

    while let Some((_, token)) = tokens.peek() {
        if *token == Token::Plus {
            tokens.next();
            let lhs = node;
            let rhs = mul(line, tokens);
            node = Box::new(Node::Binary(BinOp::Add, lhs, rhs));
        } else if *token == Token::Minus {
            tokens.next();
            let lhs = node;
            let rhs = mul(line, tokens);
            node = Box::new(Node::Binary(BinOp::Sub, lhs, rhs));
        } else {
            break;
        }
    }
    node
}

/// トークン列からmulを読み込む
///
/// mulとして解釈可能な最長の部分を取り出して抽象構文木の部分木を作る。
/// mulとは`unary ("*" unary | "/" unary)*`である。
///
/// - line - ソースコードの行。エラー表示に用いる。
/// - tokens - (usize,
/// [`Token`])のPeekableなイテレータ。usizeはトークンのソースコード中での位置である
pub fn mul<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)> + std::fmt::Debug,
{
    let mut node = unary(line, tokens);

    while let Some((_, token)) = tokens.peek() {
        if *token == Token::Asterisk {
            tokens.next();
            let lhs = node;
            let rhs = unary(line, tokens);
            node = Box::new(Node::Binary(BinOp::Mul, lhs, rhs));
        } else if *token == Token::Slash {
            tokens.next();
            let lhs = node;
            let rhs = unary(line, tokens);
            node = Box::new(Node::Binary(BinOp::Div, lhs, rhs));
        } else {
            break;
        }
    }

    node
}

/// トークン列からunaryを読み込む
///
/// unaryとして解釈可能な最長の部分を取り出して抽象構文木の部分木を作る。
/// unaryとは`("+" | "-")? primary`である。
///
/// - line - ソースコードの行。エラー表示に用いる。
/// - tokens - (usize,
/// [`Token`])のPeekableなイテレータ。usizeはトークンのソースコード中での位置である
pub fn unary<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)> + std::fmt::Debug,
{
    if let Some((_, token)) = tokens.peek() {
        if *token == Token::Plus {
            tokens.next();
            let operand = primary(line, tokens);
            return Box::new(Node::Unary(UnaryOp::Positive, operand));
        } else if *token == Token::Minus {
            tokens.next();
            let operand = primary(line, tokens);
            return Box::new(Node::Unary(UnaryOp::Negative, operand));
        }
    }
    return primary(line, tokens);
}

/// トークン列からprimaryを読み込む
///
/// primaryとして解釈可能な最長の部分を取り出して抽象構文木の部分木を作る。
/// primaryとは`num | "(" expr ")"`である。numは終端要素である。
///
/// - line - ソースコードの行。エラー表示に用いる。
/// - tokens - (usize,
/// [`Token`])のPeekableなイテレータ。usizeはトークンのソースコード中での位置である
pub fn primary<I>(line: &str, tokens: &mut Peekable<I>) -> Box<Node>
where
    I: Iterator<Item = (usize, Token)> + std::fmt::Debug,
{
    if let Some((_, Token::OpenParenthesis)) = tokens.peek() {
        tokens.next();
        let node = expr(line, tokens);
        if let Some((_, token)) = tokens.peek() {
            if *token == Token::CloseParenthesis {
                tokens.next();
                return node;
            }
        }

        show_error_panic(
            "')' がありません",
            line,
            if let Some((i, _)) = tokens.next() {
                i
            } else {
                line.len()
            },
        )
    } else if let Some((_, Token::Num(_))) = tokens.peek() {
        if let Some((_, Token::Num(num))) = tokens.next() {
            return Box::new(Node::Num(num));
        }
    }

    show_error_panic(
        "数ではありません。numも'('も見つかりませんでした",
        line,
        if let Some((i, _)) = tokens.next() {
            i
        } else {
            line.len()
        },
    );
}

#[cfg(test)]
mod test {
    use super::BinOp::*;
    use super::Node::*;
    use super::*;
    use crate::token::tokenize;

    #[test]
    fn test_expr1() {
        let s = "1+2-3";
        let mut tokens = tokenize(s).into_iter().peekable();
        assert_eq!(
            expr(s, &mut tokens),
            Box::new(Binary(
                Sub,
                Box::new(Binary(Add, Box::new(Num(1)), Box::new(Num(2)))),
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
            Box::new(Binary(
                Sub,
                Box::new(Binary(
                    Add,
                    Box::new(Num(1)),
                    Box::new(Binary(Mul, Box::new(Num(2)), Box::new(Num(3))))
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
            Box::new(Binary(
                Sub,
                Box::new(Binary(
                    Add,
                    Box::new(Num(1)),
                    Box::new(Binary(Add, Box::new(Num(2)), Box::new(Num(3))))
                )),
                Box::new(Num(4))
            ))
        );
    }
}

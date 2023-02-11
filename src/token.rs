use anyhow::bail;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Num(u32),
}

pub fn tokenize<I>(iter: &mut Peekable<I>) -> anyhow::Result<Vec<Token>>
where
    I: Iterator<Item = char>,
{
    let mut tokens = Vec::new();

    while let Some(c) = iter.peek() {
        if c.is_whitespace() {
            iter.next();
            continue;
        }

        if *c == '+' {
            iter.next();
            tokens.push(Token::Plus);
            continue;
        }
        if *c == '-' {
            iter.next();
            tokens.push(Token::Minus);
            continue;
        }
        if c.is_digit(10) {
            if let Some(num) = crate::c::strtol(iter) {
                tokens.push(Token::Num(num));
                continue;
            }
        }

        bail!("could not tokenize: {:?}", iter.peek());
    }

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        use Token::*;

        let expr = "12 + 34-45";
        let mut iter = expr.chars().peekable();
        let tokens = tokenize(&mut iter).unwrap();
        assert_eq!(tokens, vec![Num(12), Plus, Num(34), Minus, Num(45)]);
    }
}

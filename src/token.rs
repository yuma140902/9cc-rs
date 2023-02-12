use crate::chars::PeekableIterExt;
use crate::show_error_panic;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Num(u32),
}

pub fn tokenize(s: &str) -> Vec<(usize, Token)> {
    let mut iter = s.chars().enumerate().peekable();
    let mut tokens = Vec::new();

    while let Some(_) = iter.peek() {
        iter.skip_whitespaces();

        if let Some((i, _)) = iter.take_char('+') {
            tokens.push((i, Token::Plus));
            continue;
        }

        if let Some((i, _)) = iter.take_char('-') {
            tokens.push((i, Token::Minus));
            continue;
        }

        if let Some((i, num)) = iter.take_num() {
            tokens.push((i, Token::Num(num)));
            continue;
        }

        if let Some((i, _)) = iter.next() {
            show_error_panic("could not tokenize", s, i)
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        use Token::*;

        let expr = "12 + 34-45";
        let tokens = tokenize(&expr);
        assert_eq!(
            tokens,
            vec![
                (0, Num(12)),
                (3, Plus),
                (5, Num(34)),
                (7, Minus),
                (8, Num(45))
            ]
        );
    }
}

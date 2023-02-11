use anyhow::bail;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Plus,
    Minus,
    Num(u32),
}

/// ## Example
/// ```
/// use ninecc::token::tokenize;
/// use ninecc::token::Token::*;
///
/// let expr = "1 +42 - 4".to_string();
/// let tokens = tokenize(&expr).unwrap();
/// assert_eq!(tokens, vec![(0, Num(1)), (2, Plus), (3, Num(42)), (6, Minus), (8, Num(4))]);
/// ```
pub fn tokenize(s: &str) -> anyhow::Result<Vec<(usize, Token)>> {
    let mut iter = s.chars().enumerate().peekable();
    let mut tokens = Vec::new();

    while let Some((_, c)) = iter.peek() {
        if c.is_whitespace() {
            iter.next();
            continue;
        }

        if *c == '+' {
            if let Some((i, _)) = iter.next() {
                tokens.push((i, Token::Plus));
            }
            continue;
        }
        if *c == '-' {
            if let Some((i, _)) = iter.next() {
                tokens.push((i, Token::Minus));
            }
            continue;
        }
        if c.is_digit(10) {
            if let Some((i, num)) = crate::c::strtol(&mut iter) {
                tokens.push((i, Token::Num(num)));
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
        let tokens = tokenize(&expr).unwrap();
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

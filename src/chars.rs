use std::iter::Peekable;

pub trait CharsIterExt {
    fn skip_whitespaces(&mut self);
    fn take_char(&mut self, c: char) -> Option<(usize, char)>;
    fn take_ascii_digit(&mut self) -> Option<(usize, u32)>;
    fn take_num(&mut self) -> Option<(usize, u32)>;
}

impl<I> CharsIterExt for Peekable<I>
where
    I: Iterator<Item = (usize, char)>,
{
    fn skip_whitespaces(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn take_char(&mut self, c: char) -> Option<(usize, char)> {
        if let Some((_, d)) = self.peek() {
            if *d == c {
                return self.next();
            }
        }
        None
    }

    fn take_ascii_digit(&mut self) -> Option<(usize, u32)> {
        if let Some((_, c)) = self.peek() {
            if c.is_ascii_digit() {
                return self.next().map(|(i, c)| (i, c.to_digit(10).unwrap_or(0)));
            }
        }
        None
    }

    fn take_num(&mut self) -> Option<(usize, u32)> {
        let mut ret = None;
        while let Some((i, d)) = self.take_ascii_digit() {
            if ret.is_none() {
                ret = Some((i, 0));
            }
            ret = ret.map(|(i, r)| (i, r * 10 + d));
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_skip_whitespaces() {
        let s = "    \t abc";
        let mut iter = s.chars().enumerate().peekable();
        iter.skip_whitespaces();
        assert!(iter.take_char('a').is_some());
    }

    #[test]
    fn test_take_char() {
        let s = "abcd";
        let mut iter = s.chars().enumerate().peekable();
        assert_eq!(iter.take_char('a'), Some((0, 'a')));
        assert_eq!(iter.take_char('b'), Some((1, 'b')));
        assert_eq!(iter.take_char('z'), None);
    }

    #[test]
    fn test_take_ascii_digit() {
        let s = "12a34";
        let mut iter = s.chars().enumerate().peekable();
        assert_eq!(iter.take_ascii_digit(), Some((0, 1)));
        assert_eq!(iter.take_ascii_digit(), Some((1, 2)));
        assert_eq!(iter.take_ascii_digit(), None);
        assert_eq!(iter.take_ascii_digit(), None);
    }

    #[test]
    fn test_take_num() {
        let s = "12a34";
        let mut iter = s.chars().enumerate().peekable();
        assert_eq!(iter.take_num(), Some((0, 12)));
        assert_eq!(iter.take_num(), None);
    }
}

use std::iter::Peekable;

/// 文字列のイテレータを受け取り、数字以外が現れるまで進める。その間に現れた数字たちを10進数とみなして変換する。
///
/// ## Returns
/// 数字が1つも現れなかった場合はNone
///
/// ## Example
///
/// ```
/// use ninecc::c::strtol;
///
/// let s = "12a34";
/// let mut iter = s.chars().peekable();
/// assert_eq!(strtol(&mut iter), Some(12));
/// ```
///
/// ```
/// use ninecc::c::strtol;
///
/// let s = "abcd";
/// let mut iter = s.chars().peekable();
/// assert_eq!(strtol(&mut iter), None);
/// ```
///
pub fn strtol<I>(iter: &mut Peekable<I>) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    let mut ret = None;
    while let Some(c) = iter.peek() {
        if c.is_digit(10) {
            if let Some(c) = iter.next() {
                if ret.is_none() {
                    ret = Some(0);
                }
                let d = c.to_digit(10).unwrap_or(0);
                ret = ret.map(|r| r * 10 + d);
                continue;
            }
        }
        break;
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strtol1() {
        let s = "1234";
        assert_eq!(strtol(&mut s.chars().peekable()), Some(1234));
    }

    #[test]
    fn test_strtol2() {
        let s = "abc";
        assert_eq!(strtol(&mut s.chars().peekable()), None);
    }

    #[test]
    fn test_strtol3() {
        let s = "12+34";
        let mut iter = s.chars().peekable();
        assert_eq!(strtol(&mut iter), Some(12));
        eprintln!("part 1 done: pointing {:?}", iter.peek());
        iter.next(); // skip +
        assert_eq!(strtol(&mut iter), Some(34));
        eprintln!("part 2 done: pointing {:?}", iter.peek());
    }
}

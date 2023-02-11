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
/// let mut iter = s.chars().enumerate().peekable();
/// assert_eq!(strtol(&mut iter), Some((0, 12)));
/// ```
///
/// ```
/// use ninecc::c::strtol;
///
/// let s = "abcd";
/// let mut iter = s.chars().enumerate().peekable();
/// assert_eq!(strtol(&mut iter), None);
/// ```
///
pub fn strtol<I>(iter: &mut Peekable<I>) -> Option<(usize, u32)>
where
    I: Iterator<Item = (usize, char)>,
{
    let mut ret = None;
    while let Some((_, c)) = iter.peek() {
        if c.is_digit(10) {
            if let Some((i, c)) = iter.next() {
                if ret.is_none() {
                    ret = Some((i, 0));
                }
                let d = c.to_digit(10).unwrap_or(0);
                ret = ret.map(|(i, r)| (i, r * 10 + d));
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
        assert_eq!(
            strtol(&mut s.chars().enumerate().peekable()),
            Some((0, 1234))
        );
    }

    #[test]
    fn test_strtol2() {
        let s = "abc";
        assert_eq!(strtol(&mut s.chars().enumerate().peekable()), None);
    }

    #[test]
    fn test_strtol3() {
        let s = "12+34";
        let mut iter = s.chars().enumerate().peekable();
        assert_eq!(strtol(&mut iter), Some((0, 12)));
        iter.next(); // skip '+'
        assert_eq!(strtol(&mut iter), Some((3, 34)));
    }
}

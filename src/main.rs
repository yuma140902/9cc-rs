use anyhow::bail;
use anyhow::ensure;
use anyhow::Context as _;
use ninecc::token::Token;
use std::env;
use std::iter::Peekable;

fn strtol<I>(iter: &mut Peekable<I>) -> anyhow::Result<u32>
where
    I: Iterator<Item = char>,
{
    ninecc::c::strtol(iter).with_context(|| format!("invalid character: {:?}", iter.peek()))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut iter = s.chars().peekable();

    let mut tokens = ninecc::token::tokenize(&mut iter)?.into_iter();

    match tokens.next() {
        Some(Token::Num(num)) => println!("  mov rax, {}", num),
        otherwise => bail!("数ではありません: {:?}", otherwise),
    };

    while let Some(token) = tokens.next() {
        let op = if token == Token::Plus {
            "add"
        } else if token == Token::Minus {
            "sub"
        } else {
            bail!("演算子ではありません: {:?}", token)
        };

        match tokens.next() {
            Some(Token::Num(num)) => println!("  {} rax, {}", op, num),
            otherwise => bail!("数ではありません: {:?}", otherwise),
        };
    }

    println!("  ret");

    Ok(())
}

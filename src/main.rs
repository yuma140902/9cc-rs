use anyhow::bail;
use anyhow::Context as _;
use std::env;
use std::iter::Peekable;

fn strtol<I>(iter: &mut Peekable<I>) -> anyhow::Result<u32>
where
    I: Iterator<Item = char>,
{
    ninecc::strtol(iter).with_context(|| format!("invalid character: {:?}", iter.peek()))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut iter = s.chars().peekable();

    println!("  mov rax, {}", strtol(&mut iter)?);

    while let Some(c) = iter.peek() {
        let op = if *c == '+' {
            "add"
        } else if *c == '-' {
            "sub"
        } else {
            bail!("invalid character: {:?}", c);
        };

        iter.next();
        println!("  {} rax, {}", op, strtol(&mut iter)?);
    }

    println!("  ret");

    Ok(())
}

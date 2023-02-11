use anyhow::bail;
use ninecc::show_error_panic;
use ninecc::token::Token;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let mut tokens = ninecc::token::tokenize(s).into_iter();

    match tokens.next() {
        Some((_, Token::Num(num))) => println!("  mov rax, {}", num),
        Some((i, token)) => show_error_panic(format!("数ではありません: {:?}", token), s, i),
        _ => bail!("数ではありません"),
    };

    while let Some((_, token)) = tokens.next() {
        let op = if token == Token::Plus {
            "add"
        } else if token == Token::Minus {
            "sub"
        } else {
            bail!("演算子ではありません: {:?}", token)
        };

        match tokens.next() {
            Some((_, Token::Num(num))) => println!("  {} rax, {}", op, num),
            Some((i, token)) => show_error_panic(format!("数ではありません: {:?}", token), s, i),
            _ => bail!("数ではありません"),
        };
    }

    println!("  ret");

    Ok(())
}

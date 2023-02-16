use ninecc::{ast::expr, gen};
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    let mut tokens = ninecc::token::tokenize(s).into_iter().peekable();

    let ast = expr(s, &mut tokens);
    eprintln!("{:#?}", ast);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    gen::gen(&ast);
    println!("\tpop rax");
    println!("\tret");

    Ok(())
}

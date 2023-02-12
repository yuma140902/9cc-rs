use ninecc::ast::expr;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    let mut tokens = ninecc::token::tokenize(s).into_iter().peekable();

    let ast = expr(s, &mut tokens);
    dbg!(ast);

    Ok(())
}

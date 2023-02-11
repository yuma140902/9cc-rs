use ninecc::strtol;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");

    println!(
        "\
.intel_syntax noprefix
.globl main
main:"
    );

    let mut iter = s.chars().peekable();

    println!("  mov rax, {}", strtol(&mut iter).unwrap());

    loop {
        if let Some(c) = iter.peek() {
            if *c == '+' {
                iter.next();
                println!("  add rax, {}", strtol(&mut iter).unwrap());
                continue;
            }
            if *c == '-' {
                iter.next();
                println!("  sub rax, {}", strtol(&mut iter).unwrap());
                continue;
            }
        }
        break;
    }

    println!("  ret");
}

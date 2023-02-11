use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let s = args.get(1).expect("Usage: 9ccrs INPUT");
    let val: i32 = s.trim().parse().expect("error not a number");
    println!(
        "\
.intel_syntax noprefix
.globl main
main:
\tmov rax, {}
\tret",
        val
    );
}

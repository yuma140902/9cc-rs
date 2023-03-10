pub mod ast;
pub mod chars;
pub mod gen;
pub mod token;

pub fn show_error(msg: &str, line: &str, index: usize) {
    eprintln!("{}", line);
    for _ in 0..index {
        eprint!(" ");
    }
    eprintln!("^");
    eprintln!("{}", msg);
}

pub fn show_error_panic(msg: &str, line: &str, index: usize) -> ! {
    show_error(msg, line, index);
    panic!();
}

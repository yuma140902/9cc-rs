pub mod c;
pub mod token;

pub fn show_error(msg: &str, line: &str, index: usize) {
    eprintln!("{}", msg);
    eprintln!("{}", line);
    for _ in 0..index {
        eprint!(" ");
    }
    eprintln!("^");
}

pub fn show_error_panic(msg: &str, line: &str, index: usize) -> ! {
    show_error(msg, line, index);
    panic!();
}

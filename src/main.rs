use std::io::{self, Write};

fn fn2(n: &mut i16) {
    *n = 4;
    println!("Hi {}", n);
}

fn main() {
    io::stdout().write(b"Please enter your name: ");
    io::stdout().flush();
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    println!("Hello, {}!", name);
}

use std::io;
use std::process;

fn main() {
    let mut sum = 0;
    loop {
        let mut number_str = String::new();
        match io::stdin().read_line(&mut number_str) {
            Ok(n) => { if n == 0 {break} },
            Err(e) => { println!("ERROR: got '{}' when reading a line", e) }
        }
        match number_str.trim().parse::<i32>() {
            Ok(n) => { sum += n }
            Err(n) => {
                println!("ERROR: Entered something that is not a number: '{}'",
                    number_str.trim_right());
                process::exit(1)
            },
        }
    }
    println!("{}", sum);
}

fn fn2(mut n: i16) {
    n = 4;
    println!("Hi {}", n);
}

fn main() {
    let mut n = 3;
    for i in 1..10 {
        println!("Hi!");
    }
    fn2(n);
    println!("Hi {}", n);
}

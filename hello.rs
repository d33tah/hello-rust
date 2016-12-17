fn fn2(n: i16) {
    println!("Hi {}", n);
}

fn main() {
    let n = 3;
    for i in 1..10 {
        println!("Hi!");
    }
    fn2(n);
}

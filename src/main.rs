fn fn2(n: &mut i16) {
    *n = 4;
    println!("Hi {}", n);
}

fn main() {
    let mut n = 3;
    for i in 1..10 {
        println!("Hi!");
    }
    println!("Hi {}", n);
    fn2(&mut n);
    println!("Hi {}", n);
}

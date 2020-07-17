use std::io;

fn main() {
    loop {
        let mut index = String::new();
        println!("Type the nth Fibonacci number: ");
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line !");
        let index: u32 = match index.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("your typed: {}", index);
        fib(index);
    }
}
fn fib(n: u32) -> u32 {
    // let mut fib_0 = 0;
    // let mut fib_1 = 1;
    let mut fib_n_2 = 0;
    let mut fib_n_1 = 1;
    let mut fib_current = 1;
    for i in 0..n {
        fib_current = fib_n_2 + fib_n_1;
        fib_n_1 = fib_n_2;
        fib_n_2 = fib_current;
    }
    println!("nth Fibonacci number: {}", fib_current);
    fib_current
}

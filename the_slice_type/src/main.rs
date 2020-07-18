//use std::io;

fn main() {
    println!("The Slice Type !");
    let s = String::from("Hello, World !");
    let length = s.len();
    let s1 = &s[2..length];
    let s2 = &s[3..10];
    let s3 = &s[5..];
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

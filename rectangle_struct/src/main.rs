use std::io;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    loop {
        let mut width = String::new();
        let mut height = String::new();
        println!("Enter the width of the rectangle:");
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line !");
        let width: u32 = match width.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("Enter the height of the rectangle:");
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line !");
        let height: u32 = match height.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        let rect1 = Rectangle { width, height };
        println!("rect1 is {:?}", rect1);
        println!("square 3x3 is {:?}", Rectangle::square(3));
        println!("area of rect1 is {}", rect1.area());
        // println!("area of rect1 is {}", area(&rect1));
    }
}
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

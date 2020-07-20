use std::io;
fn main() {
    let vector1 = vec![1, 2, 3, 4, 5];
    loop {
        let mut index = String::new();
        println!("Type the vector index that you want to display:");
        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line");
        let index: u32 = match index.trim().parse() {
            Ok(number) => {
                println!("you have entered: {}", number);
                number
            }
            Err(_) => continue,
        };
        match vector1.get(index as usize) {
            Some(&element) => println!("the {}th element is: {}", index, element),
            None => println!("Sorry, element is out of bound !"),
        };
    }
}

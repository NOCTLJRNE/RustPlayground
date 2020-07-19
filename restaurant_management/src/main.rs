mod front_of_house;
// Using a ";" rather than "{}" tells Rust to load contents ofthe module from another file with the same name

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {
    println!("A table !");
    eat_at_restaurant();
}

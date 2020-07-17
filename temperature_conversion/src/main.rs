use std::io;

fn main() {
    loop {
        let mut temp_value = String::new();
        let mut temp_unit = String::new();
        println!("Choose your temperature unit, please enter f or c:");
        io::stdin()
            .read_line(&mut temp_unit)
            .expect("Failed to read line");
        let temp_unit = temp_unit.trim();
        if temp_unit != "f" && temp_unit != "c" {
            continue;
        }
        println!("Enter the temperature value:");
        io::stdin()
            .read_line(&mut temp_value)
            .expect("Failed to read line");
        let temp_value: f32 = match temp_value.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        // let temp_converted = if temp_unit == "f" {f_to_c(temp_value)} else {c_to_f(temp_value)};
        // println!("temp unit {}", temp_unit);
        if temp_unit == "f" {
            println!("Convert from F to C");
            f_to_c(temp_value);
        } else if temp_unit == "c" {
            c_to_f(temp_value);
        }
    }
}
fn f_to_c(f: f32) -> f32 {
    println!("Convert from F to C");
    let temp_converted: f32 = (f - 32.0) / 1.8;
    println!("F: {} <=> C: {}", f, temp_converted);
    temp_converted
}
fn c_to_f(c: f32) -> f32 {
    println!("Convert from C to F");
    let temp_converted: f32 = c * 1.8 + 32.0;
    println!("C: {} <=> F: {}", c, temp_converted);
    temp_converted
}

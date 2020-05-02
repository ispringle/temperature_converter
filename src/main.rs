use std::io::{stdin, stdout, Write};
mod temperature;
use crate::temperature::Temperature;

fn main() {
    println!("Converting from Fahrenheit (f) or Celsius (c)?");
    print!("Enter your selection: [f|c]: ");
    let _ = stdout().flush();
    let mut units = String::new();
    loop {
        stdin()
            .read_line(&mut units)
            .expect("Failed to read line");
        let base = match units.trim() {
            "f" => Temperature::from_fahrenheit,
            "c" => Temperature::from_celsius,
            _ => panic!("Not f or c")
        };
    let mut temp = String::new();
        print!("Enter value to convert: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: Temperature = match temp.trim().parse() {
            Ok(num) => base(Temperature::new(), num),
            Err(_) => continue,
        };
        match units.trim() {
            "f" => println!("{}F is {}C", temp.fahrenheit, temp.celsius),
            "c" => println!("{}C is {}F", temp.celsius, temp.fahrenheit),
            _ => println!("Unknown unit type: {}", units)
    }
        break
    }
}

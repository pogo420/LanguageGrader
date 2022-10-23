// modules must be defined in main, so it can be used in other files
mod data_structures;

use crate::data_structures::sandwich::sandwich;

fn main() {
    println!("Hello, world!");
    let sw = sandwich::new(32, String::from("Boom"),  String::from("Ola 43"));
    println!("{}",sw.to_string());
}

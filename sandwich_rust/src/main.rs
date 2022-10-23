mod data_structures;

use crate::data_structures::sandwich::sandwich;

fn main() {
    println!("Hello, world!");
    let sw = sandwich::new(32, String::from("Boom"),  String::from("Ola 43"));
    println!("{}",sw.to_string());
}

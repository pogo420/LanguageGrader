/*
* Modules must be defined in main(using mod) or before using, we must define module.
* As main is in the root better we define the module using mod keyword.
* Its like importing the module, saying the compiler to save it in cache.
* With use keyword we just tell the compiler to use it.
 */
mod data_structures;
mod storage_interface;
mod utils;
mod exceptions;
mod persistance_engine;


use crate::data_structures::sandwich::sandwich;

fn main() {
    println!("Hello, world!");
    let sw = sandwich::new(32, String::from("Boom"),  String::from("Ola 43"));
    println!("{}",sw.to_string());
}
/*
* Modules must be defined in main(using mod) or before using, we must define module.
* As main is in the root better we define the module using mod keyword.
* Its like importing the module, saying the compiler to save it in cache.
* With use keyword we just tell the compiler to use it.
 */

 #![allow(unused)] // telling to compiler to prevent warning of unused code.
mod data_structures;
mod storage_interface;
mod utils;
mod exceptions;
mod persistance_engine;
mod storage_impl;


use std::io;
use persistance_engine::PersistanceEngine;
use storage_impl::json_writer::JsonWriter;
use storage_impl::json_reader::JsonReader;

fn main() {

    let mut inp = String::new();

    while inp.trim() != "q" {
        inp.clear();
        println!("Enter your command: s->save/c->create/u->update sandwiches, q to quit");
        io::stdin().read_line(&mut inp).expect("failed to readline");

        //TODO user input implementation with actual logic
        PersistanceEngine::save_sandwich(String::from("sand_name"), String::from("sand_recipe"), &JsonWriter::default());
        PersistanceEngine::update_sandwich(String::from("sand_name"), String::from("sand_recipe"), &JsonReader::default(), &JsonWriter::default());
        PersistanceEngine::delete_sandwich(String::from("sand_name"), &JsonWriter::default(), &JsonReader::default());
        PersistanceEngine::check_sandwich(String::from("sand_name"), &JsonReader::default());
    }
}

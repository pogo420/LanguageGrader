use crate::storage_interface::writer::writer::Writer;
use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::utils::PersistanceResponse;
use crate::exceptions::PersistanceFileException;

use std::fs::{OpenOptions, File};


#[derive(Default)]
pub struct JsonWriter{file_path: String}

fn write_file(file_path: String)  -> Result<File, PersistanceFileException> {
    //private function to handle writing file
    let file = OpenOptions::new()
                                        .read(true)
                                        .append(true)
                                        .write(false)
                                        .open(file_path);

    if file.is_err(){
        println!("Issue in reading file, details:{}", file.unwrap_err());
        return Err(PersistanceFileException{});
    } else {
        return Ok(file.unwrap());
    }
}

impl Writer for JsonWriter {
    fn new(file_path: String) -> Self {
        return JsonWriter{file_path: file_path};
    }
    fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        // TODO actual implementation        
        return PersistanceResponse::Success;
    }
    fn update_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        // TODO actual implementation
        return PersistanceResponse::Success;
    }
    fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        // TODO actual implementation
        return PersistanceResponse::Success;
    }
}
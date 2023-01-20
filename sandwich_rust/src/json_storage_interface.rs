use crate::storage_interface::StorageInterface;
use crate::data_structures::sandwich::sandwich::{Sandwich, new};
use crate::data_structures::sandwich_collection::SandwichCollection;
use crate::utils::{PersistanceResponse, read_file, write_file};

use serde_json;


#[derive(Default)]
pub struct JsonStorageInterface {
    file_path: String
}

fn read_sandwich_collections(file_path: &str) -> Result<SandwichCollection, serde_json::Error> {
    // Private function to handle reading sandwich collection from json file

    let data = read_file(file_path);
    return serde_json::from_str(&data);
}

fn write_sadwich_collections(file_path: &str, sandwiches: &Vec<Sandwich>) -> PersistanceResponse {
    // Private function to handle writing sandwich collection from json file

    let data_final = serde_json::to_string_pretty(&sandwiches);

    if data_final.is_err(){
        return PersistanceResponse::Failure;
    }
    else {
        if write_file(&file_path, &data_final.unwrap()) <1 {
            return PersistanceResponse::Failure ;
        }
        else {
            return PersistanceResponse::Success;
        }
    }
}

impl StorageInterface for JsonStorageInterface {

    fn setup(file_path: String) -> JsonStorageInterface {
        return JsonStorageInterface{file_path: file_path};
    }

    fn get_sandwich(&self, name: String) -> Sandwich {
        // Method to implement get sandwich

        let serde_response:Result<SandwichCollection, serde_json::Error> = read_sandwich_collections(&self.file_path);

        if serde_response.is_err(){
            return new(String::new(), String::new());
        }

        for sandwich in serde_response.unwrap().sandwiches{
            if sandwich.name == name {
                return new(name, sandwich.recipie);
            }
        }

        return new(String::new(), String::new());
    }
    
    fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        // Method to implement saving sandwich

        let serde_response:Result<SandwichCollection, serde_json::Error> = read_sandwich_collections(&self.file_path);

        if serde_response.is_err(){
            return PersistanceResponse::Failure;
        }

        let mut sandwiches = serde_response.unwrap().sandwiches;

        let mut vec = Vec::new();
        vec.push(sandwich);

        sandwiches.append(&mut vec);

        return write_sadwich_collections(&self.file_path, &sandwiches);

    }
    
    fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        // Method to implement logic to delete sandwich
        let serde_response:Result<SandwichCollection, serde_json::Error> = read_sandwich_collections(&self.file_path);

        if serde_response.is_err(){
            return PersistanceResponse::Failure;
        }

        let mut sandwiches = serde_response.unwrap().sandwiches;
        
        let mut count = 0;
        for sad in &sandwiches {
            if sad.name == sandwich.name {
               break;
            }
            count +=1;
            println!("{}",count);
        }
        sandwiches.remove(count); 

        return write_sadwich_collections(&self.file_path, &sandwiches);
    }

}
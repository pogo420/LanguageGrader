use crate::storage_interface::StorageInterface;
use crate::data_structures::sandwich::sandwich::{Sandwich, new};
use crate::data_structures::sandwich_collection::SandwichCollection;
use crate::utils::{PersistanceResponse, read_file, write_file};

use serde_json;

/*
TODO
add test cases + use solid
 */ 


#[derive(Default)]
pub struct JsonStorageInterface {
    file_path: String
}

impl StorageInterface for JsonStorageInterface {

    fn setup(file_path: String) -> JsonStorageInterface {
        return JsonStorageInterface{file_path: file_path};
    }

    fn get_sandwich(&self, name: String) -> Sandwich {
        // Method to implement get sandwich
        
        let data = read_file(&self.file_path);
        let serde_response:Result<SandwichCollection, serde_json::Error> = serde_json::from_str(&data);

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
        // method to implement saving sandwich

        let data = read_file(&self.file_path);
        let serde_response:Result<SandwichCollection, serde_json::Error> = serde_json::from_str(&data);

        if serde_response.is_err(){
            return PersistanceResponse::Failure;
        }

        let mut sandwiches = serde_response.unwrap().sandwiches;

        let mut vec = Vec::new();
        vec.push(sandwich);

        sandwiches.append(&mut vec);

        let data_final = serde_json::to_string_pretty(&sandwiches);

        if data_final.is_err(){
            return PersistanceResponse::Failure;
        }
        else {
            if write_file(&self.file_path, &data_final.unwrap()) <1 {
                return PersistanceResponse::Failure ;
            }
            else {
                return PersistanceResponse::Success;
            }
        }
    }

	fn update_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        _ = sandwich;
        return PersistanceResponse::Success;
    }
    
    fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        _ = sandwich;
        return PersistanceResponse::Success;
    }

}
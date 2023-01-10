use crate::storage_interface::StorageInterface;
use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::utils::PersistanceResponse;

#[derive(Default)]
pub struct JsonStorageInterface {
    file_path: String
}

impl StorageInterface for JsonStorageInterface {

    fn setup(file_path: String) -> JsonStorageInterface {
        return JsonStorageInterface{file_path: file_path};
    }

    fn get_sandwich(&self, name: String) -> Sandwich {
        _= *self.file_path;
        _ = name;
        return Sandwich{name: String::from("dummy"), recipie: String::from("dummy")};
    }
    
    fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
        _ = sandwich;
        return PersistanceResponse::Success;
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
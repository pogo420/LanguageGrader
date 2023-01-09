use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::utils::PersistanceResponse;


pub trait StorageInterface {
        fn setup(&self, file_path: String) -> Self;
        fn get_sandwich(&self, name: String) -> Sandwich;
        fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
	fn update_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
        fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
}


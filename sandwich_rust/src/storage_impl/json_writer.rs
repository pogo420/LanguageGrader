use crate::storage_interface::writer::writer::Writer;
use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::utils::PersistanceResponse;

// TODO write test cases

#[derive(Default)]
pub struct JsonWriter{file_path: String}

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
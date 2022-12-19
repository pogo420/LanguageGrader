use crate::storage_interface::writer::writer::Writer;
use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::utils::PersistanceResponse;

// TODO write test cases
pub struct JsonWriter{}

impl Writer for JsonWriter {
    fn new() -> Self {
        return JsonWriter{};
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
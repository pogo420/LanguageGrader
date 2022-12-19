use crate::storage_interface::reader::reader::Reader;
use crate::data_structures::sandwich::sandwich::Sandwich;

// TODO write test cases

pub struct JsonReader{}

impl Reader for JsonReader{
    fn get_sandwich(&self, name: String) -> Sandwich {
        // TODO actual implementation
        return Sandwich{name: name, recipie: String::from("Sample recipie")};
    }
    fn new() -> Self {
        return JsonReader{};
    }
}
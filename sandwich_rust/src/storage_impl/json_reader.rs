use crate::storage_interface::reader::reader::Reader;
use crate::data_structures::sandwich::sandwich::Sandwich;

// TODO write test cases

#[derive(Default)]
pub struct JsonReader{file_path: String}

impl Reader for JsonReader{
    fn get_sandwich(&self, name: String) -> Sandwich {
        // TODO actual implementation
        &self.file_path;
        return Sandwich{name: name, recipie: String::from("Sample recipie")};
    }
    fn new(file_path: String) -> Self {
        return JsonReader{file_path: file_path};
    }
}
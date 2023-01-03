use crate::storage_interface::reader::reader::Reader;
use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::data_structures::sandwich_collection::SandwichCollection;
use crate::exceptions::PersistanceFileException;

use std::fs::File;
use std::io::BufReader;

// TODO write test cases

#[derive(Default)]
pub struct JsonReader{file_path: String}


fn read_file(file_name: &str) -> Result<BufReader<File>, PersistanceFileException> {
    // private function to read file
    let file = File::open(file_name);

    if file.is_err(){
        println!("Issue in reading file, details:{}", file.unwrap_err());
        return Err(PersistanceFileException{});
    } else {
        let reader = BufReader::new(file.unwrap());
        return Ok(reader);
    }
}


impl Reader for JsonReader{
    fn get_sandwich(&self, name: String) -> Sandwich {

        let rdr = read_file(&self.file_path);

        if rdr.is_err(){

            return Sandwich::default();
        }
        let result_json:Result<SandwichCollection, serde_json::Error> = serde_json::from_reader(rdr.unwrap());

        if result_json.is_err(){
            println!("Issue in json parsing..{:?}", result_json.err());
            return Sandwich::default();
        }
        let sandwiches : SandwichCollection = result_json.unwrap();
        
        for sw in sandwiches.sandwiches{
            if sw.name == name {
                return Sandwich{name: sw.name, recipie: sw.recipie};
            } 
        }
        return Sandwich::default();

    }
    fn new(file_path: String) -> Self {
        return JsonReader{file_path: file_path};
    }
}
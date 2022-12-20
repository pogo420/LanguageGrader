
use sandwich_rust::Reader;
use sandwich_rust::Sandwich;
use sandwich_rust::new;
use sandwich_rust::PersistanceResponse;
use sandwich_rust::Writer;
use sandwich_rust::is_empty_string;


// Test case for reader interface sanity
#[test]
fn check_reader_interface() {
    struct TestR {
        file_name: String
    }
    impl Reader for TestR {
        fn new(file_name: String) -> Self{
            return TestR{file_name: file_name};
        }
        fn get_sandwich(&self, name: String) -> Sandwich {
            _ = &self.file_name;
            return new(name,  String::from("Ola 43"));
        }
    }
    let tr: TestR = Reader::new(String::from("/file/to/data"));
    assert_eq!(tr.get_sandwich(String::from("HelloSandwich")).name, "HelloSandwich");
}

// Test case for writer interface sanity
#[test]
fn check_writer_interface() {
    struct TestW {
        file_path: String
    }
    impl Writer for TestW {
        fn new(file_path: String) -> Self {
            return TestW{file_path: file_path};
        }
        fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
            if is_empty_string(&sandwich.name){
                return PersistanceResponse::Success;
            }
            else {
                return PersistanceResponse::Failure;
            }
        }
        fn update_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse {
            if is_empty_string(&sandwich.name){
                return PersistanceResponse::Success;
            }
            else {
                return PersistanceResponse::Failure;
            }
        }
        fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse 
        {
            if is_empty_string(&sandwich.name){
                return PersistanceResponse::Failure;
            }
            else {
                _ = &self.file_path;
                return PersistanceResponse::Success;
            }
        }
    }
    let tw: TestW = Writer::new(String::from("/file/to/data"));
    let s = Sandwich{
        name: String::from("Boom"),
        recipie: String::from("Ola 32")
    };

    matches!(tw.save_sandwich(s), PersistanceResponse::Success);
}
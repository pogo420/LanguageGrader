
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
    }
    impl Reader for TestR {
        fn new() -> Self{
            return TestR{};
        }
        fn get_sandwich(&self, name: String) -> Sandwich {
            return new(name,  String::from("Ola 43"));
        }
    }
    let tr: TestR = Reader::new();
    assert_eq!(tr.get_sandwich(String::from("HelloSandwich")).name, "HelloSandwich");
}

// Test case for writer interface sanity
#[test]
fn check_writer_interface() {
    struct TestW {
    }
    impl Writer for TestW {
        fn new() -> Self {
            return TestW{};
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
                return PersistanceResponse::Success;
            }
        }
    }
    let tw: TestW = Writer::new();
    let s = Sandwich{
        name: String::from("Boom"),
        recipie: String::from("Ola 32")
    };

    matches!(tw.save_sandwich(s), PersistanceResponse::Success);
}

use sandwich_rust::Reader;
use sandwich_rust::Sandwich;
use sandwich_rust::new;
use sandwich_rust::PersistanceResponse;
use sandwich_rust::Writer;

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
            return new(32, name,  String::from("Ola 43"));
        }
    }
    let tr: TestR = Reader::new();
    assert_eq!(tr.get_sandwich(String::from("HelloSandwich")).id, 32);
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
            if sandwich.id > 0{
                return PersistanceResponse::Success;
            }
            else {
                return PersistanceResponse::Failure;
            }
        }
        fn update_sandwich(&self, sandwich: Sandwich, id: u8) -> PersistanceResponse {
            if sandwich.id > 0 && id > 0{
                return PersistanceResponse::Success;
            }
            else {
                return PersistanceResponse::Failure;
            }
        }
    }
    let tw: TestW = Writer::new();
    let s = Sandwich{
        id: 32,
        name: String::from("Boom"),
        recipie: String::from("Ola 32")
    };

    matches!(tw.save_sandwich(s), PersistanceResponse::Success);
}
/*
 * Interface to be implemented for writing sandwich persistance reader model
 */
pub mod reader {
    use super::super::super::data_structures::sandwich::sandwich::Sandwich;
    pub trait Reader{
        fn new() -> Self; // self for making an function method, Self for current object, trait object. Imp: Self vs self as class vs this
        fn get_sandwich(&self, name: String) -> Sandwich;
    }
}


/*
* Section for unit test cases
*/
#[cfg(test)]
mod tests {
    use super::reader::Reader;
    use super::super::super::data_structures::sandwich::sandwich::{new, Sandwich};

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
}

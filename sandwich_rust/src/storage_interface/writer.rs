/*
 * Interface to be implemented for writing sandwich persistance writer model  
 */
pub mod writer {
    use super::super::super::data_structures::sandwich::sandwich::Sandwich;
    use super::super::super::utils::PersistanceResponse;

    pub trait Writer {
        fn new() -> Self;
        fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
	    fn update_sandwich(&self, sandwich: Sandwich, id: u8) -> PersistanceResponse;
    }
}

/*
* Section for unit test cases
*/
#[cfg(test)]
mod tests {
    use super::writer::Writer;
    use super::super::super::data_structures::sandwich::sandwich::Sandwich;
    use super::super::super::utils::PersistanceResponse;

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

        assert_eq!(tw.save_sandwich(s), PersistanceResponse::Success);




    }
}

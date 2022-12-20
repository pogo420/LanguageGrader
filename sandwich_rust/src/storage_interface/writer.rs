/*
 * Interface to be implemented for writing sandwich persistance writer model  
 */
pub mod writer {
    use super::super::super::data_structures::sandwich::sandwich::Sandwich;
    use super::super::super::utils::PersistanceResponse;

    pub trait Writer {
        fn new(file_path: String) -> Self;
        fn save_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
	    fn update_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
        fn delete_sandwich(&self, sandwich: Sandwich) -> PersistanceResponse;
    }
}

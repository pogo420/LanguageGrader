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

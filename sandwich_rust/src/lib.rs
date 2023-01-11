/* 
 * Using lib.rs is must for test cases in test folder to use src functions
 * As tests code do not have direct access to src code.
 * It must be done via lib.rs
 */
pub mod data_structures;
pub use data_structures::sandwich::sandwich::{new, Sandwich};
pub use data_structures::sandwich_collection::SandwichCollection;

pub mod storage_interface;
pub use storage_interface::StorageInterface;

pub mod json_storage_interface;
pub use json_storage_interface::JsonStorageInterface;

pub mod exceptions;

pub mod utils;
pub use utils::{PersistanceResponse, is_empty_string, read_file, write_file};
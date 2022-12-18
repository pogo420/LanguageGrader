/* 
 * Using lib.rs is must for test cases in test folder to use src functions
 * As tests code do not have direct access to src code.
 * It must be done via lib.rs
 */
pub mod data_structures;
pub use data_structures::sandwich::sandwich::new;
pub use data_structures::sandwich::sandwich::Sandwich;

pub mod storage_interface;
pub use storage_interface::reader::reader::Reader;
pub use storage_interface::writer::writer::Writer;

pub mod utils;
pub use utils::PersistanceResponse;
pub use utils::is_empty_string;

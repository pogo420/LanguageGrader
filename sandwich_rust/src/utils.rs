use std::{fs::{read_to_string, write}};
/*
 * Set of utility function for the application usage
 */

// persistance response
pub enum PersistanceResponse {
    Success,
    Failure,
    Unknown
}

pub fn is_empty_string(data: &str) -> bool{
    // function to handle empty string
    if (*data).is_empty() || (*data).eq("") {
        return true;
    }
    else {
        return false;
    }
}

pub fn read_file(file_path: &str) -> String {
    // Function to read file
    let file_result = read_to_string(file_path);
    if file_result.is_ok(){
        return file_result.unwrap();
    }
    else {
        return String::new();
    }
}

pub fn write_file(file_path: &str, data: &str) -> u8{
    // function to write into a file, Returns zero in case of issues.
    let file_result = write(file_path, data);
    if file_result.is_ok(){
        return 1;
    }
    else {
        return 0;
    }
}
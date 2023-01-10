use std::{fs::read_to_string};

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

pub fn read_file(file_path: String) -> String {
    // Function to read file
    let file_result = read_to_string(file_path);
    if file_result.is_ok(){
        return file_result.unwrap();
    }
    else {
        return String::new();
    }
}
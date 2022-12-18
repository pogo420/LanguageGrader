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
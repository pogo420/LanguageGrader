use sandwich_rust::{is_empty_string, read_file, write_file};

#[test]
fn validate_is_empty_string() {
    /*
     *  Function to validate the is_empty_string
     */
    let str1: String = String::from("");
    let str2: String = String::new();
    let str3: String = String::from("Hello");

    assert_eq!(is_empty_string(&str1), true);
    assert_eq!(is_empty_string(&str2), true);
    assert_eq!(is_empty_string(&str3), false);
}

#[test]
fn valid_read_file() {
    /*
    Test case for basic read operation check
    */
    let data = read_file("tests/test_data/file_read_test.txt");
    assert_eq!(data, "1 2\n3 4\n");
}

#[test]
fn invalid_read_file() {
    /*
    Negative Test case for basic read operation check
    */
    let data = read_file("FILE_NOT_EXISTS");
    assert_eq!(data, "");
}

#[test]
fn valid_write_file() {
    /*
    Test case for basic write operation check
    */
    let data = "1 2\n3 4\n";
    let response = write_file("tests/test_data/file_write_test.txt", data);

    assert_eq!(response, 1);
    
}
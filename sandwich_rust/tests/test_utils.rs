use sandwich_rust::is_empty_string;

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

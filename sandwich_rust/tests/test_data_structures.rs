
use sandwich_rust::new;

// Test case for Sandwich data structure sanity
#[test]
fn check_sandwich_object() {
    let sw = new(String::from("Boom"),  String::from("Ola 43"));
    assert_eq!("Boom", sw.name);
    assert_eq!("Ola 43", sw.recipie);
} 

// Test case for Sandwich stringer
#[test]
fn check_sandwich_string() {
    let sw = new(String::from("Boom"),  String::from("Ola 43"));

    let response = sw.to_string();
    let expected = format!("Sandwich \n\tName:{},\n\tRecipe:{}","Boom",  "Ola 43");

    assert_eq!(response, expected);
    }

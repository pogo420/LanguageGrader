
use sandwich_rust::new;

// Test case for Sandwich data structure sanity
#[test]
fn check_sandwich_object() {
    let sw = new(32, String::from("Boom"),  String::from("Ola 43"));
    assert_eq!(32, sw.id);
    assert_eq!("Boom", sw.name);
    assert_eq!("Ola 43", sw.recipie);
} 

// Test case for Sandwich stringer
#[test]
fn check_sandwich_string() {
    let sw = new(32, String::from("Boom"),  String::from("Ola 43"));

    let response = sw.to_string();
    let expected = format!("Sandwich \n\tId:{},\n\tName:{},\n\tRecipe:{}", 32, "Boom",  "Ola 43");

    assert_eq!(response, expected);
    }

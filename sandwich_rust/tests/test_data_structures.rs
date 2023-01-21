
use sandwich_rust::new;
use sandwich_rust::SandwichCollection;


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

#[test]
fn check_sandwich_collection_object() {
    let swv = vec![
        new(String::from("Boom1"),  String::from("Ola 43")), 
        new(String::from("Boom2"),  String::from("Ola 43"))
        ];
    let sws = SandwichCollection{sandwiches: swv};
    let mut count = 1;
    for s in sws.sandwiches{
        assert_eq!(s.name,format!("Boom{}",count));
        count+=1;
    }
}
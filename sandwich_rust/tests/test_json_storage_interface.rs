use sandwich_rust::JsonStorageInterface;
use sandwich_rust::StorageInterface;

#[test]
fn test_json_read_valid(){
    // test case to check sanity of json read

    let interface:  JsonStorageInterface = StorageInterface::setup(String::from("tests/test_data/valid_json.json"));
    let sandwich = interface.get_sandwich(String::from("dummy_sand_1"));

    assert_eq!(sandwich.recipie, "Boom Boom 1");
}

#[test]
fn test_json_read_invalid(){
// negative test case to check sanity of json read
    let interface:  JsonStorageInterface = StorageInterface::setup(String::from("tests/test_data/valid_json.json"));
    let sandwich = interface.get_sandwich(String::from("not present"));

    assert_eq!(sandwich.recipie, "");
}
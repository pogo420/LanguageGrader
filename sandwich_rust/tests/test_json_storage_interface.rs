use sandwich_rust::JsonStorageInterface;
use sandwich_rust::StorageInterface;
use sandwich_rust::new;
use sandwich_rust::PersistanceResponse;

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

#[test]
fn test_save_sandwich(){
// test case to check sanity of json data save
    let interface:  JsonStorageInterface = StorageInterface::setup(String::from("tests/test_data/valid_save_json.json"));
    let sandwich = new(String::from("ola"), String::from("recipe from ola"));

    let response = interface.save_sandwich(sandwich);

    matches!(response, PersistanceResponse::Success);
}

#[test]
fn test_delete_sandwich(){
// test case to check sanity of json data save
    let interface:  JsonStorageInterface = StorageInterface::setup(String::from("tests/test_data/valid_save_json.json"));
    let sandwich = new(String::from("ola"), String::from("recipe from ola"));
    interface.save_sandwich(new(String::from("ola"), String::from("recipe from ola")));
    let response = interface.delete_sandwich(sandwich);

    matches!(response, PersistanceResponse::Success);
}
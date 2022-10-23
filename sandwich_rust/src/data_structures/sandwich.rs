/*
 * Sandwich data structure
 */

pub mod sandwich {  // module containing sandwitch ds(Abstrction), Without public we can't use it.
    // struct
    pub struct Sandwich {
        pub id: u8,
        pub name: String,
        pub recipie: String
    }
    // implementations/logic for sandwich ops
    impl Sandwich {
        pub fn to_string(&self) -> String {
            return format!("Sandwich \n\tId:{},\n\tName:{},\n\tRecipe:{}",self.id, self.name, self.recipie);
        }
        fn new(id: u8, name: String, recipie: String) -> Sandwich{
            return Sandwich{
                id,
                name,
                recipie
            };
        }
    }
    // Public APIs
    pub fn new(id: u8, name: String, recipie: String) -> Sandwich {
        return Sandwich::new(id, name, recipie);
    }
}

/*
* Section for unit test cases
*/
#[cfg(test)]
mod tests {
    use super::sandwich::new;

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
}

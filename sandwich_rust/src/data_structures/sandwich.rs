/*
 * Sandwich data structure
 */

pub mod sandwich {  // module containing sandwitch ds(Abstrction), Without public we can't use it.
    
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Default)]  // for json support
    pub struct Sandwich {
        pub name: String,
        pub recipie: String
    }
    // implementations/logic for sandwich ops
    impl Sandwich {
        pub fn to_string(&self) -> String {
            return format!("Sandwich \n\tName:{},\n\tRecipe:{}",self.name, self.recipie);
        }
        fn new(name: String, recipie: String) -> Sandwich{
            return Sandwich{
                name,
                recipie
            };
        }
    }
    // Public APIs
    pub fn new(name: String, recipie: String) -> Sandwich {
        return Sandwich::new(name, recipie);
    }
}

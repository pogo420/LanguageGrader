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

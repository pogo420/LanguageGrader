use crate::data_structures::sandwich::sandwich::Sandwich;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(transparent)]  // allows to encapsulate your data with your type
// DS to save sandwich data in json format
pub struct SandwichCollection {
    pub sandwiches: Vec<Sandwich>
}
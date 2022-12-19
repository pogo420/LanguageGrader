use crate::data_structures::sandwich::sandwich::Sandwich;
use crate::data_structures::sandwich::sandwich;
use crate::exceptions::exceptions::SandwichNotFoundException;
use crate::exceptions::exceptions::InvalidSandwichData;
use crate::storage_interface::reader::reader::Reader;
use crate::storage_interface::writer::writer::Writer;
use crate::utils::PersistanceResponse;
use crate::utils::is_empty_string;
 

pub struct PersistanceEngine {

}

impl PersistanceEngine {

    pub fn check_sandwich<T: Reader>(name : String, reader: &T) -> Result<Sandwich, SandwichNotFoundException>
    {
        // method to get a sandwich
        if name == "" || name.is_empty(){
            return Err(SandwichNotFoundException{});
        }
        else {
            return Ok(reader.get_sandwich(name));
        }
    }

    pub fn save_sandwich<T: Writer>(sand_name : String, 
                                    sand_recipie: String , 
                                    writer: &T) -> Result<PersistanceResponse, InvalidSandwichData>
                                    {
        // method to create a sandwich
        
        if is_empty_string(&sand_name) || is_empty_string(&sand_recipie) {
            return Err(InvalidSandwichData{});
        }
        else {
            let sw: Sandwich = sandwich::new(sand_name, sand_recipie);
            return Ok(writer.save_sandwich(sw));
        }
    }

    pub fn update_sandwich<T: Writer, Q: Reader>(sand_name : String, 
                                    sand_recipie: String ,
                                    reader: &Q,
                                    writer: &T) -> Result<PersistanceResponse, InvalidSandwichData>
                                    {
        // method to create a sandwich
        
        if is_empty_string(&sand_name) || is_empty_string(&sand_recipie) {
            return Err(InvalidSandwichData{});
        }
        else {
            let mut sw: Sandwich = reader.get_sandwich(sand_name);
            sw.recipie = sand_recipie;
            return Ok(writer.save_sandwich(sw));
        }
    }

    pub fn delete_sandwich<T: Writer, Q: Reader>(sand_name : String, writer: &T, reader: &Q){
        // method to delete a sandwich app

        let sw: Sandwich = reader.get_sandwich(sand_name);
        writer.delete_sandwich(sw);
        
    }

}

use crate::pet_module::PetState;

mod pet_module;

use pet_module::*;
fn main() {
    let ps = {
        let ps = PetState::new();

        let (ps, id) = Dog::new(ps, "Shippen".to_string(), "gray".to_string(), "schnauzer".to_string(), true, 2.0, 42);
        let (ps, id) = Dog::new(ps, "Sophie".to_string(), "blond".to_string(), "schnauzer".to_string(), true, 2.0, 56);
        let (ps, id) = Dog::new(ps, "Waterloo".to_string(), "blond".to_string(), "labrador".to_string(), true, 12.0, 4);
        let (ps, id) = Cat::new(ps, "Berlioz".to_string(), "black".to_string(), "shorthair".to_string(), true, true, false);
        let (ps, id) = Cat::new(ps, "Simba".to_string(), "blond".to_string(), "shorthair".to_string(),true,true,true);
        ps
    };
    let x = ps.get_all_dogs();
    x.iter().for_each(|dog|{println!("{:?}", dog);});

    println!("Blonds");
    // let x:Vec<Mammal>
}

pub(crate) use crate::pet_module::pet_state::PetState;

mod pet_state;

#[derive(Debug)]
pub(crate) struct Dog {
    uuid: String,
    name: String,
    hair_color: String,
    breed: String,
    has_hair: bool,
    tail_length: f64,
    num_commands_known: i32,
}

pub(crate) struct Cat {
    uuid: String,
    name: String,
    hair_color: String,
    breed: String,
    has_hair: bool,
    declawed: bool,
    sits_on_keyboard: bool,
}

impl Dog {
    pub(crate) fn new(
        mut ps: PetState,
        name: String,
        hair_color: String,
        breed: String,
        has_hair: bool,
        tail_length: f64,
        num_commands_known: i32,
    ) -> (PetState,String) {
        let id = ps.add_dog(
            name.clone(),
            hair_color.clone(),
            breed.clone(),
            has_hair,
            tail_length,
            num_commands_known,
        );
        (ps,id)
    }
}

impl Cat {
    pub(crate) fn new(
        mut ps: PetState,
        name: String,
        hair_color: String,
        breed: String,
        has_hair: bool,
        declawed: bool,
        sits_on_keyboard: bool,
    ) -> (PetState, String) {
        let id = ps.add_cat(
            name.clone(),
            hair_color.clone(),
            breed.clone(),
            has_hair,
            declawed,
            sits_on_keyboard,
        );
        (ps, id)
    }
}

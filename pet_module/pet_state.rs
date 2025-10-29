use crate::pet_module::Dog;
use specs::prelude::*;
use specs_derive::Component;

pub(crate) struct PetState {
    pub(crate) ecs: World,
}
impl PetState {
    pub(crate) fn new() -> Self {
        let mut ps = PetState { ecs: World::new() };
        ps.ecs.register::<PetComponent>();
        ps.ecs.register::<MammalComponent>();
        ps.ecs.register::<ReptileComponent>();
        ps.ecs.register::<DogComponent>();
        ps.ecs.register::<CatComponent>();
        ps.ecs.register::<TurtleComponent>();
        ps.ecs.register::<SnakeComponent>();
        ps
    }

    pub(in crate::pet_module) fn add_dog(
        &mut self,
        name: String,
        hair_color: String,
        breed: String,
        has_hair: bool,
        tail_length: f64,
        num_commands_known: i32,
    ) -> String {
        let uuid = "1234".to_string();
        self.ecs
            .create_entity()
            .with(PetComponent {uuid: uuid.clone(), name})
            .with(MammalComponent {hair_color, breed, has_hair})
            .with(DogComponent {tail_length, num_commands_known})
            .build();
        uuid
    }

    pub(in crate::pet_module) fn add_cat(
        &mut self,
        name: String,
        hair_color: String,
        breed: String,
        has_hair: bool,
        declawed: bool,
        sits_on_keyboard: bool,
    )  -> String {
    let uuid = "1234".to_string();
        self.ecs
            .create_entity()
            .with(PetComponent {
                uuid: uuid.clone(),
                name,
            })
            .with(MammalComponent {
                hair_color,
                breed,
                has_hair,
            })
            .with(CatComponent {
                declawed,
                sits_on_keyboard,
            })
            .build();
        uuid
    }

    pub(crate) fn get_all_dogs(&self) -> Vec<Dog> {
        let mut dogs = Vec::new();
        let fetched_dogs = self.ecs.read_storage::<DogComponent>();
        let fetched_pets = self.ecs.read_storage::<PetComponent>();
        let fetched_mammals = self.ecs.read_storage::<MammalComponent>();
        for (dog, mammal, pet) in (&fetched_dogs, &fetched_mammals, &fetched_pets).join() {
            dogs.push(Dog {
                uuid: pet.uuid.clone(),
                name: pet.name.clone(),
                hair_color: mammal.hair_color.clone(),
                breed: mammal.breed.clone(),
                has_hair: mammal.has_hair,
                tail_length: dog.tail_length,
                num_commands_known: dog.num_commands_known,
            });
        }
        dogs
    }
    // pub(crate) fn get_dogs(&self) -> Vec<String> {
    //     let dogs = self.ecs.read_storage::<DogComponent>();
    //
    // }
}

#[derive(Component, Debug)]
struct PetComponent {
    pub(crate) uuid: String,
    name: String,
}

#[derive(Component, Debug)]
struct MammalComponent {
    hair_color: String,
    breed: String,
    has_hair: bool,
}

#[derive(Component, Debug)]
struct ReptileComponent {
    scale_color: String,
    is_poisonous: bool,
}

#[derive(Component, Debug)]
struct DogComponent {
    tail_length: f64,
    num_commands_known: i32,
}

#[derive(Component, Debug)]
struct CatComponent {
    declawed: bool,
    sits_on_keyboard: bool,
}
#[derive(Component, Debug)]
struct TurtleComponent {
    is_aquatic: bool,
    is_soft_shelled: bool,
}

#[derive(Component, Debug)]
struct SnakeComponent {
    length: f64,
}

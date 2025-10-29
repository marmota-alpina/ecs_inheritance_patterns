//! Demonstration program for ECS-based Class Table Inheritance.
//!
//! This binary demonstrates all features of the `ecs_inheritance_patterns` crate:
//! - Creating pets using composition
//! - Querying by specific type (dogs, cats)
//! - Polymorphic queries (all mammals)
//! - Filtered queries (by hair color)
//! - Accessing composed data structures
//! - Pattern matching with exhaustive checking
//! - Using convenience methods
//! - Display trait formatting
//!
//! # Running
//!
//! ```bash
//! cargo run
//! ```
//!
//! Or for optimized build:
//!
//! ```bash
//! cargo run --release
//! ```

use crate::pet_module::PetState;

pub mod pet_module;

use pet_module::*;

fn main() {
    println!("=== ECS-Based Inheritance Pattern Demonstration ===\n");

    println!("Step 1: Creating PetState and adding pets...");

    let ps = {
        let ps = PetState::new();

        let (ps, _shippen_id) = Dog::create(ps, "Shippen", "gray", "schnauzer", true, 2.0, 42);

        let (ps, _sophie_id) = Dog::create(ps, "Sophie", "blond", "schnauzer", true, 2.0, 56);

        let (ps, _waterloo_id) = Dog::create(ps, "Waterloo", "blond", "labrador", true, 12.0, 4);

        let (ps, _berlioz_id) = Cat::create(ps, "Berlioz", "black", "shorthair", true, true, false);

        let (ps, _simba_id) = Cat::create(ps, "Simba", "blond", "shorthair", true, true, true);

        println!("Created 3 dogs and 2 cats\n");

        ps
    };

    println!("Step 2: Querying all dogs (simulates JOIN across Pet, Mammal, Dog tables)");
    println!("{}", "-".repeat(70));

    let dogs = ps.get_all_dogs();
    for dog in &dogs {
        println!(
            "Dog: {} ({}), Breed: {}, Commands Known: {}",
            dog.pet.name,
            dog.mammal.hair_color,
            dog.mammal.breed,
            dog.dog_specific.num_commands_known
        );
    }
    println!();

    println!("Step 3: Querying all cats");
    println!("{}", "-".repeat(70));

    let cats = ps.get_all_cats();
    for cat in &cats {
        println!(
            "Cat: {} ({}), Sits on Keyboard: {}",
            cat.pet.name, cat.mammal.hair_color, cat.cat_specific.sits_on_keyboard
        );
    }
    println!();

    println!("Step 4: Querying all mammals (polymorphic collection via enum)");
    println!("{}", "-".repeat(70));

    let all_mammals = ps.get_all_mammals();
    println!("Total mammals: {}", all_mammals.len());

    for mammal in &all_mammals {
        // Access common data through the enum's methods
        let pet_data = mammal.pet_data();
        let mammal_data = mammal.mammal_data();
        let sound = mammal.make_sound();

        println!(
            "{}: {} ({}) - Says: {}",
            match mammal {
                Mammal::Dog(_) => "Dog",
                Mammal::Cat(_) => "Cat",
            },
            pet_data.name,
            mammal_data.hair_color,
            sound
        );
    }
    println!();

    println!("Step 5: Filtered query - All blond mammals");
    println!("{}", "-".repeat(70));

    let blond_mammals = ps.get_mammals_by_hair_color("blond");
    for mammal in &blond_mammals {
        println!(
            "{}: {}",
            match mammal {
                Mammal::Dog(_) => "Dog",
                Mammal::Cat(_) => "Cat",
            },
            mammal.pet_data().name
        );
    }
    println!();

    println!("Step 6: Demonstrating composition structure");
    println!("{}", "-".repeat(70));

    if let Some(dog) = dogs.first() {
        println!("Accessing dog data through composition:");
        println!("  dog.pet.uuid = {}", dog.pet.uuid);
        println!("  dog.pet.name = {}", dog.pet.name);
        println!("  dog.mammal.breed = {}", dog.mammal.breed);
        println!("  dog.mammal.has_hair = {}", dog.mammal.has_hair);
        println!(
            "  dog.dog_specific.tail_length = {}",
            dog.dog_specific.tail_length
        );
        println!("\nThis structure makes the Class Table Inheritance mapping explicit!");
    }

    if let Some(cat) = cats.first() {
        println!("\nAccessing cat data:");
        println!("  cat.pet.uuid = {}", cat.pet.uuid);
        println!("  cat.pet.name = {}", cat.pet.name);
        println!(
            "  cat.cat_specific.declawed = {}",
            cat.cat_specific.declawed
        );
    }
    println!();

    println!("Step 7: Using convenience methods on Mammal enum");
    println!("{}", "-".repeat(70));

    for mammal in all_mammals.iter().take(3) {
        println!("Name: {}", mammal.name());
        println!("UUID: {}", mammal.uuid());
        println!("Hair Color: {}", mammal.hair_color());
        println!("Is Dog? {}", mammal.is_dog());
        println!("Is Cat? {}", mammal.is_cat());

        // Type-specific access using as_dog() and as_cat()
        if let Some(dog) = mammal.as_dog() {
            println!("Commands Known: {}", dog.dog_specific.num_commands_known);
        }
        if let Some(cat) = mammal.as_cat() {
            println!("Sits on Keyboard: {}", cat.cat_specific.sits_on_keyboard);
        }
        println!();
    }

    println!("Step 8: Using Display trait for formatted output");
    println!("{}", "-".repeat(70));

    println!("Dogs:");
    for dog in &dogs {
        println!("  {}", dog);
    }

    println!("\nCats:");
    for cat in &cats {
        println!("  {}", cat);
    }

    println!("\nAll Mammals:");
    for mammal in all_mammals.iter().take(3) {
        println!("  {}", mammal);
    }
    println!();

    println!("Step 9: Pattern matching with exhaustive checking");
    println!("{}", "-".repeat(70));

    for mammal in all_mammals.iter().take(2) {
        match mammal {
            Mammal::Dog(dog) => {
                println!(
                    "This is a dog named {} who knows {} commands",
                    dog.pet.name, dog.dog_specific.num_commands_known
                );
            }
            Mammal::Cat(cat) => {
                println!(
                    "This is a cat named {} - Keyboard sitter: {}",
                    cat.pet.name, cat.cat_specific.sits_on_keyboard
                );
            }
        }
    }
    println!();

    println!("{}", "=".repeat(70));
    println!("Summary of Solutions:");
    println!("{}", "=".repeat(70));
    println!("1. Field duplication solved through composition (PetData, MammalData)");
    println!("2. Vec<Mammal> type safety solved through enum-based polymorphism");
    println!("3. ECS components remain hidden, maintaining clean architecture");
    println!("4. Clear mapping to Class Table Inheritance pattern");
    println!("5. Zero-cost abstractions (no vtable, no heap allocation for enums)");
    println!();
    println!("Modern Rust Features (2024):");
    println!("6. impl Into<String> for ergonomic APIs accepting &str and String");
    println!("7. From trait implementations for clean type conversions");
    println!("8. Display trait for user-friendly output");
    println!("9. Convenience methods for better ergonomics (name, is_dog, as_dog, etc.)");
    println!("10. Iterator chaining for functional query patterns");
    println!("11. Default trait for PetState initialization");
    println!("{}", "=".repeat(70));
}

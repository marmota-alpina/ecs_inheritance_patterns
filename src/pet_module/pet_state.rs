//! Private implementation layer using ECS components.
//!
//! This module implements the **Gateway Pattern** from Fowler, providing
//! a clean separation between the domain model and persistence strategy.
//!
//! # Architecture
//!
//! - **Components** map to database tables in Class Table Inheritance
//! - **Entities** represent rows with implicit foreign key relationships
//! - **Joins** simulate SQL JOIN operations across multiple tables
//!
//! # Component Mapping
//!
//! ```text
//! PetComponent      → Pet table (id, name)
//! MammalComponent   → Mammal table (pet_id, hair_color, breed, has_hair)
//! DogComponent      → Dog table (mammal_id, tail_length, num_commands_known)
//! CatComponent      → Cat table (mammal_id, declawed, sits_on_keyboard)
//! ```
//!
//! # Extension Components
//!
//! The module includes placeholder components (`ReptileComponent`, `TurtleComponent`,
//! `SnakeComponent`) to demonstrate extensibility. These are registered but not
//! yet implemented. See README section "Extending the Project" for exercises.

use crate::pet_module::{Cat, CatData, Dog, DogData, Mammal, MammalData, PetData};
use specs::prelude::*;
use specs_derive::Component;
use uuid::Uuid;

// ============================================================================
// ECS Components (Hidden Implementation Detail)
// These map directly to database tables in Class Table Inheritance pattern
// ============================================================================

/// Component representing the Pet table (base of inheritance hierarchy).
///
/// Maps to: `Pet` table with columns (id, name)
///
/// This is the root component that all pets share, equivalent to the
/// base table in Class Table Inheritance.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct PetComponent {
    pub(crate) uuid: String,
    name: String,
}

impl From<&PetComponent> for PetData {
    fn from(comp: &PetComponent) -> Self {
        Self {
            uuid: comp.uuid.clone(),
            name: comp.name.clone(),
        }
    }
}

/// Component representing the Mammal table (intermediate hierarchy level).
///
/// Maps to: `Mammal` table with columns (id, pet_id, hair_color, breed, has_hair)
///
/// This component extends `PetComponent` through entity composition,
/// representing the middle level of the inheritance hierarchy.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct MammalComponent {
    hair_color: String,
    breed: String,
    has_hair: bool,
}

impl From<&MammalComponent> for MammalData {
    fn from(comp: &MammalComponent) -> Self {
        Self {
            hair_color: comp.hair_color.clone(),
            breed: comp.breed.clone(),
            has_hair: comp.has_hair,
        }
    }
}

/// Placeholder component for Reptile hierarchy (educational extension).
///
/// Maps to: `Reptile` table with columns (id, pet_id, scale_color, is_poisonous)
///
/// This component is registered but not yet implemented, demonstrating
/// how to extend the pattern to support multiple inheritance hierarchies.
/// See README section "Extending the Project" → "Adding Different Hierarchies".
///
/// **Exercise:** Implement `Turtle` and `Snake` types using this component,
/// similar to how `Dog` and `Cat` use `MammalComponent`.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
#[allow(dead_code)]
struct ReptileComponent {
    scale_color: String,
    is_poisonous: bool,
}

/// Component representing the Dog table (leaf level in hierarchy).
///
/// Maps to: `Dog` table with columns (id, mammal_id, tail_length, num_commands_known)
///
/// This is a concrete type at the leaf of the inheritance tree, representing
/// dog-specific attributes. Combined with `PetComponent` and `MammalComponent`
/// via entity composition to form a complete `Dog`.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct DogComponent {
    tail_length: f64,
    num_commands_known: i32,
}

impl From<&DogComponent> for DogData {
    fn from(comp: &DogComponent) -> Self {
        Self {
            tail_length: comp.tail_length,
            num_commands_known: comp.num_commands_known,
        }
    }
}

/// Component representing the Cat table (leaf level in hierarchy).
///
/// Maps to: `Cat` table with columns (id, mammal_id, declawed, sits_on_keyboard)
///
/// This is a concrete type at the leaf of the inheritance tree, representing
/// cat-specific attributes. Combined with `PetComponent` and `MammalComponent`
/// via entity composition to form a complete `Cat`.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct CatComponent {
    declawed: bool,
    sits_on_keyboard: bool,
}

impl From<&CatComponent> for CatData {
    fn from(comp: &CatComponent) -> Self {
        Self {
            declawed: comp.declawed,
            sits_on_keyboard: comp.sits_on_keyboard,
        }
    }
}

/// Placeholder component for Turtle type (educational extension).
///
/// Maps to: `Turtle` table with columns (id, reptile_id, is_aquatic, is_soft_shelled)
///
/// Part of the suggested Reptile hierarchy extension. Would be used alongside
/// `PetComponent` and `ReptileComponent` to form a complete `Turtle` type.
///
/// **Exercise:** Implement `Turtle` struct in pet_module.rs and add creation/query methods.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
#[allow(dead_code)]
struct TurtleComponent {
    is_aquatic: bool,
    is_soft_shelled: bool,
}

/// Placeholder component for Snake type (educational extension).
///
/// Maps to: `Snake` table with columns (id, reptile_id, length)
///
/// Part of the suggested Reptile hierarchy extension. Would be used alongside
/// `PetComponent` and `ReptileComponent` to form a complete `Snake` type.
///
/// **Exercise:** Implement `Snake` struct in pet_module.rs and add creation/query methods.
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
#[allow(dead_code)]
struct SnakeComponent {
    length: f64,
}

// ============================================================================
// PetState: The Gateway/Data Access Layer
// Encapsulates ECS implementation and provides clean domain API
// ============================================================================

/// Gateway to the pet storage system.
///
/// `PetState` encapsulates the ECS (Entity-Component-System) implementation
/// and provides a clean API for creating and querying pets.
///
/// # Architecture
///
/// This struct implements the **Gateway Pattern** from Fowler, separating
/// the domain model from persistence concerns.
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::*;
///
/// let ps = PetState::new();
/// let (ps, dog_id) = Dog::create(ps, "Max", "brown", "boxer", true, 10.0, 15);
///
/// let dogs = ps.get_all_dogs();
/// assert_eq!(dogs.len(), 1);
/// ```
pub struct PetState {
    pub(crate) ecs: World,
}

impl Default for PetState {
    fn default() -> Self {
        Self::new()
    }
}

impl PetState {
    /// Creates a new `PetState` with all component types registered.
    ///
    /// This initializes the underlying ECS world and registers all
    /// component types needed for the pet hierarchy.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::PetState;
    ///
    /// let ps = PetState::new();
    /// // Ready to add pets
    /// ```
    pub fn new() -> Self {
        let mut ps = PetState { ecs: World::new() };

        // Register all component types with the ECS world
        ps.ecs.register::<PetComponent>();
        ps.ecs.register::<MammalComponent>();
        ps.ecs.register::<ReptileComponent>();
        ps.ecs.register::<DogComponent>();
        ps.ecs.register::<CatComponent>();
        ps.ecs.register::<TurtleComponent>();
        ps.ecs.register::<SnakeComponent>();

        ps
    }

    // ========================================================================
    // Write Operations (Create)
    // ========================================================================

    /// Adds a dog to the ECS world, creating an entity with three components.
    ///
    /// Creates an entity with:
    /// - `PetComponent` (base data: uuid, name)
    /// - `MammalComponent` (mammal data: hair_color, breed, has_hair)
    /// - `DogComponent` (dog-specific: tail_length, num_commands_known)
    ///
    /// This simulates inserting rows into three related tables in Class Table Inheritance.
    ///
    /// # Returns
    ///
    /// Returns the UUID of the created dog for future reference.
    ///
    /// # Database Equivalent
    ///
    /// ```sql
    /// INSERT INTO Pet (id, name) VALUES (uuid, 'Buddy');
    /// INSERT INTO Mammal (id, pet_id, hair_color, breed, has_hair)
    ///   VALUES (uuid2, uuid, 'brown', 'labrador', true);
    /// INSERT INTO Dog (id, mammal_id, tail_length, num_commands_known)
    ///   VALUES (uuid3, uuid2, 10.5, 15);
    /// ```
    pub(in crate::pet_module) fn add_dog(
        &mut self,
        name: impl Into<String>,
        hair_color: impl Into<String>,
        breed: impl Into<String>,
        has_hair: bool,
        tail_length: f64,
        num_commands_known: i32,
    ) -> String {
        let uuid = Uuid::new_v4().to_string();
        let name = name.into();
        let hair_color = hair_color.into();
        let breed = breed.into();

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
            .with(DogComponent {
                tail_length,
                num_commands_known,
            })
            .build();

        uuid
    }

    /// Adds a cat to the ECS world, creating an entity with three components.
    ///
    /// Creates an entity with:
    /// - `PetComponent` (base data: uuid, name)
    /// - `MammalComponent` (mammal data: hair_color, breed, has_hair)
    /// - `CatComponent` (cat-specific: declawed, sits_on_keyboard)
    ///
    /// This simulates inserting rows into three related tables in Class Table Inheritance.
    ///
    /// # Returns
    ///
    /// Returns the UUID of the created cat for future reference.
    ///
    /// # Database Equivalent
    ///
    /// ```sql
    /// INSERT INTO Pet (id, name) VALUES (uuid, 'Whiskers');
    /// INSERT INTO Mammal (id, pet_id, hair_color, breed, has_hair)
    ///   VALUES (uuid2, uuid, 'orange', 'tabby', true);
    /// INSERT INTO Cat (id, mammal_id, declawed, sits_on_keyboard)
    ///   VALUES (uuid3, uuid2, false, true);
    /// ```
    pub(in crate::pet_module) fn add_cat(
        &mut self,
        name: impl Into<String>,
        hair_color: impl Into<String>,
        breed: impl Into<String>,
        has_hair: bool,
        declawed: bool,
        sits_on_keyboard: bool,
    ) -> String {
        let uuid = Uuid::new_v4().to_string();
        let name = name.into();
        let hair_color = hair_color.into();
        let breed = breed.into();

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

    // ========================================================================
    // Read Operations (Query)
    // These methods demonstrate the "Gateway" pattern and lazy loading concept
    // ========================================================================

    /// Retrieves all dogs from the ECS world.
    ///
    /// Performs a join query across Pet, Mammal, and Dog components,
    /// simulating a SQL JOIN across three tables in Class Table Inheritance.
    ///
    /// # SQL Equivalent
    ///
    /// ```sql
    /// SELECT * FROM Pet
    /// INNER JOIN Mammal ON Pet.id = Mammal.pet_id
    /// INNER JOIN Dog ON Mammal.id = Dog.mammal_id
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Buddy", "brown", "mutt", true, 8.0, 10);
    /// let (ps, _) = Dog::create(ps, "Max", "black", "lab", true, 12.0, 20);
    ///
    /// let dogs = ps.get_all_dogs();
    /// assert_eq!(dogs.len(), 2);
    /// ```
    pub fn get_all_dogs(&self) -> Vec<Dog> {
        let fetched_dogs = self.ecs.read_storage::<DogComponent>();
        let fetched_pets = self.ecs.read_storage::<PetComponent>();
        let fetched_mammals = self.ecs.read_storage::<MammalComponent>();

        // Join query - only entities with all three components
        // Using From trait implementations for cleaner conversions
        (&fetched_dogs, &fetched_mammals, &fetched_pets)
            .join()
            .map(|(dog, mammal, pet)| Dog {
                pet: pet.into(),
                mammal: mammal.into(),
                dog_specific: dog.into(),
            })
            .collect()
    }

    /// Retrieves all cats from the ECS world.
    ///
    /// Similar to [`get_all_dogs`](Self::get_all_dogs), performs a join query
    /// across Pet, Mammal, and Cat components.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Cat::create(ps, "Whiskers", "orange", "tabby", true, false, true);
    /// let (ps, _) = Cat::create(ps, "Shadow", "black", "bombay", true, true, false);
    ///
    /// let cats = ps.get_all_cats();
    /// assert_eq!(cats.len(), 2);
    /// ```
    pub fn get_all_cats(&self) -> Vec<Cat> {
        let fetched_cats = self.ecs.read_storage::<CatComponent>();
        let fetched_pets = self.ecs.read_storage::<PetComponent>();
        let fetched_mammals = self.ecs.read_storage::<MammalComponent>();

        // Join query - using From trait implementations for cleaner conversions
        (&fetched_cats, &fetched_mammals, &fetched_pets)
            .join()
            .map(|(cat, mammal, pet)| Cat {
                pet: pet.into(),
                mammal: mammal.into(),
                cat_specific: cat.into(),
            })
            .collect()
    }

    /// Retrieves all mammals as a polymorphic collection.
    ///
    /// This demonstrates the solution to the `Vec<Mammal>` type safety concern
    /// by using enum variants for type-safe polymorphism.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Buddy", "brown", "retriever", true, 11.0, 15);
    /// let (ps, _) = Cat::create(ps, "Mittens", "white", "persian", true, false, true);
    /// let (ps, _) = Dog::create(ps, "Max", "black", "poodle", true, 8.0, 20);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert_eq!(mammals.len(), 3);
    ///
    /// // All mammals share common interface
    /// for mammal in mammals {
    ///     println!("{} says {}", mammal.name(), mammal.make_sound());
    /// }
    /// ```
    pub fn get_all_mammals(&self) -> Vec<Mammal> {
        // Collect all dogs and cats using iterator chaining
        // More functional and efficient than creating intermediate collections
        self.get_all_dogs()
            .into_iter()
            .map(Mammal::Dog)
            .chain(self.get_all_cats().into_iter().map(Mammal::Cat))
            .collect()
    }

    /// Gets mammals filtered by hair color.
    ///
    /// Demonstrates how business logic can be implemented at the gateway layer.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Goldie", "golden", "retriever", true, 12.0, 15);
    /// let (ps, _) = Cat::create(ps, "Shadow", "black", "bombay", true, false, true);
    /// let (ps, _) = Dog::create(ps, "Gold", "golden", "labrador", true, 10.0, 18);
    ///
    /// let golden_mammals = ps.get_mammals_by_hair_color("golden");
    /// assert_eq!(golden_mammals.len(), 2);
    /// ```
    pub fn get_mammals_by_hair_color(&self, hair_color: &str) -> Vec<Mammal> {
        self.get_all_mammals()
            .into_iter()
            .filter(|m| m.mammal_data().hair_color == hair_color)
            .collect()
    }

    /// Retrieves a specific dog by UUID (demonstrates lazy loading pattern).
    ///
    /// This method demonstrates how to implement efficient single-entity queries,
    /// which would be the foundation for lazy loading patterns in larger applications.
    ///
    /// # Performance
    ///
    /// Optimized to use `.find()` which stops at the first match, rather than
    /// fetching all dogs and filtering. This is more efficient for large datasets.
    ///
    /// # Returns
    ///
    /// Returns `Some(Dog)` if found, `None` if no dog with that UUID exists.
    ///
    /// # Example Use Case
    ///
    /// In a web application, you might load a dog by ID from a URL parameter:
    /// ```ignore
    /// let dog = ps.get_dog_by_id(&url_param)?;
    /// ```
    #[allow(dead_code)]
    pub(crate) fn get_dog_by_id(&self, uuid: &str) -> Option<Dog> {
        let fetched_dogs = self.ecs.read_storage::<DogComponent>();
        let fetched_pets = self.ecs.read_storage::<PetComponent>();
        let fetched_mammals = self.ecs.read_storage::<MammalComponent>();

        // Direct query for specific UUID - stops at first match
        (&fetched_dogs, &fetched_mammals, &fetched_pets)
            .join()
            .find(|(_, _, pet)| pet.uuid == uuid)
            .map(|(dog, mammal, pet)| Dog {
                pet: pet.into(),
                mammal: mammal.into(),
                dog_specific: dog.into(),
            })
    }
}

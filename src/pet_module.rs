//! Pet module implementing Fowler's Class Table Inheritance pattern.
//!
//! This module demonstrates how to model inheritance patterns in Rust using
//! composition and enums, mapping to database table structures through ECS components.
//!
//! # Architecture
//!
//! The module is split into two layers:
//! - **Public API** (this file): Domain model with `Dog`, `Cat`, and `Mammal` types
//! - **Private implementation** (pet_state): ECS components and query logic (Gateway pattern)
//!
//! # Example
//!
//! ```
//! use ecs_inheritance_patterns::pet_module::*;
//!
//! // Create a PetState (data access layer)
//! let ps = PetState::new();
//!
//! // Add a dog using composition
//! let (ps, dog_id) = Dog::create(
//!     ps,
//!     "Buddy",
//!     "brown",
//!     "labrador",
//!     true,
//!     10.5,
//!     15
//! );
//!
//! // Query all dogs
//! let dogs = ps.get_all_dogs();
//! assert_eq!(dogs.len(), 1);
//! assert_eq!(dogs[0].pet.name, "Buddy");
//! ```

pub use crate::pet_module::pet_state::PetState;

mod pet_state;

use std::fmt;

// ============================================================================
// Data Structures using Composition (addresses field duplication concern)
// ============================================================================

/// Base data shared by all pets.
///
/// Maps to the `Pet` table in Class Table Inheritance pattern.
/// This is the root of the inheritance hierarchy.
///
/// # Fields
///
/// * `uuid` - Unique identifier for the pet
/// * `name` - Pet's name
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::PetData;
///
/// let pet_data = PetData {
///     uuid: "123e4567-e89b-12d3-a456-426614174000".to_string(),
///     name: "Fluffy".to_string(),
/// };
///
/// assert_eq!(pet_data.name, "Fluffy");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PetData {
    pub uuid: String,
    pub name: String,
}

/// Data shared by all mammals.
///
/// Maps to the `Mammal` table in Class Table Inheritance pattern.
/// In a database, this would have a foreign key to the `Pet` table.
///
/// # Fields
///
/// * `hair_color` - Color of the mammal's hair/fur
/// * `breed` - Breed classification
/// * `has_hair` - Whether the mammal has hair (true for most mammals)
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::MammalData;
///
/// let mammal_data = MammalData {
///     hair_color: "golden".to_string(),
///     breed: "retriever".to_string(),
///     has_hair: true,
/// };
///
/// assert!(mammal_data.has_hair);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MammalData {
    pub hair_color: String,
    pub breed: String,
    pub has_hair: bool,
}

/// Data specific to reptiles.
///
/// Maps to the `Reptile` table in Class Table Inheritance pattern.
/// Included for demonstration purposes and student exercises.
///
/// # Note
///
/// This type is not currently used in the main implementation but demonstrates
/// how to extend the hierarchy to support multiple animal types.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReptileData {
    pub scale_color: String,
    pub is_poisonous: bool,
}

/// Dog-specific data.
///
/// Maps to the `Dog` table in Class Table Inheritance pattern.
/// In a database, this would have a foreign key to the `Mammal` table.
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::DogData;
///
/// let dog_data = DogData {
///     tail_length: 12.5,
///     num_commands_known: 20,
/// };
///
/// assert_eq!(dog_data.num_commands_known, 20);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DogData {
    pub tail_length: f64,
    pub num_commands_known: i32,
}

/// Cat-specific data.
///
/// Maps to the `Cat` table in Class Table Inheritance pattern.
/// In a database, this would have a foreign key to the `Mammal` table.
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::CatData;
///
/// let cat_data = CatData {
///     declawed: false,
///     sits_on_keyboard: true,
/// };
///
/// assert!(cat_data.sits_on_keyboard);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatData {
    pub declawed: bool,
    pub sits_on_keyboard: bool,
}

// ============================================================================
// Domain Model: Concrete Types using Composition
// ============================================================================

/// A dog entity using composition to avoid field duplication.
///
/// This structure demonstrates how composition can replace inheritance
/// for data reuse. Instead of inheriting from Pet → Mammal → Dog,
/// we compose `Dog` from three data structures.
///
/// # Database Mapping
///
/// This maps to a Class Table Inheritance pattern with three tables:
/// - `Pet` table (uuid, name)
/// - `Mammal` table (pet_id FK, hair_color, breed, has_hair)
/// - `Dog` table (mammal_id FK, tail_length, num_commands_known)
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::*;
///
/// let ps = PetState::new();
/// let (ps, dog_id) = Dog::create(
///     ps,
///     "Max",
///     "black",
///     "german_shepherd",
///     true,
///     15.0,
///     25
/// );
///
/// let dogs = ps.get_all_dogs();
/// assert_eq!(dogs[0].pet.name, "Max");
/// assert_eq!(dogs[0].mammal.breed, "german_shepherd");
/// assert_eq!(dogs[0].dog_specific.num_commands_known, 25);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Dog {
    pub pet: PetData,
    pub mammal: MammalData,
    pub dog_specific: DogData,
}

/// A cat entity using composition.
///
/// Like [`Dog`], this demonstrates composition-based data reuse.
/// The structure mirrors the three-table database design.
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::*;
///
/// let ps = PetState::new();
/// let (ps, cat_id) = Cat::create(
///     ps,
///     "Whiskers",
///     "orange",
///     "tabby",
///     true,
///     false,
///     true
/// );
///
/// let cats = ps.get_all_cats();
/// assert_eq!(cats[0].pet.name, "Whiskers");
/// assert!(cats[0].cat_specific.sits_on_keyboard);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Cat {
    pub pet: PetData,
    pub mammal: MammalData,
    pub cat_specific: CatData,
}

// ============================================================================
// Polymorphic Type: Enum-based Solution (addresses Vec<Mammal> concern)
// ============================================================================

/// Type-safe polymorphic collection of mammals.
///
/// This enum demonstrates Rust's idiomatic alternative to inheritance-based
/// polymorphism. Instead of `Vec<Box<dyn Animal>>`, we use `Vec<Mammal>`.
///
/// # Benefits
///
/// - **Type safety**: `Vec<Mammal>` works without trait objects
/// - **Zero-cost**: No heap allocation, no vtable lookup
/// - **Exhaustiveness**: Compiler ensures all cases are handled
/// - **Performance**: Pattern matching compiles to direct dispatch
///
/// # Database Mapping
///
/// Maps naturally to Single Table Inheritance with a discriminant column:
/// ```sql
/// CREATE TABLE Mammals (
///     id UUID PRIMARY KEY,
///     type VARCHAR,  -- 'Dog' or 'Cat' (discriminant)
///     name VARCHAR,
///     hair_color VARCHAR,
///     -- Dog fields
///     tail_length FLOAT,
///     num_commands INT,
///     -- Cat fields
///     declawed BOOLEAN,
///     sits_on_keyboard BOOLEAN
/// );
/// ```
///
/// # Example
///
/// ```
/// use ecs_inheritance_patterns::pet_module::*;
///
/// let ps = PetState::new();
/// let (ps, _) = Dog::create(ps, "Buddy", "brown", "beagle", true, 8.0, 10);
/// let (ps, _) = Cat::create(ps, "Mittens", "white", "persian", true, false, true);
///
/// // Type-safe polymorphic collection
/// let mammals: Vec<Mammal> = ps.get_all_mammals();
/// assert_eq!(mammals.len(), 2);
///
/// for mammal in mammals {
///     match mammal {
///         Mammal::Dog(dog) => println!("Dog: {}", dog.pet.name),
///         Mammal::Cat(cat) => println!("Cat: {}", cat.pet.name),
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Mammal {
    Dog(Dog),
    Cat(Cat),
}

impl Mammal {
    /// Returns a reference to the common [`PetData`].
    ///
    /// This method demonstrates how to access shared data across enum variants
    /// without requiring trait objects or inheritance.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Rex", "brown", "boxer", true, 10.0, 15);
    ///
    /// let mammals = ps.get_all_mammals();
    /// let pet_data = mammals[0].pet_data();
    /// assert_eq!(pet_data.name, "Rex");
    /// ```
    pub fn pet_data(&self) -> &PetData {
        match self {
            Mammal::Dog(d) => &d.pet,
            Mammal::Cat(c) => &c.pet,
        }
    }

    /// Returns a reference to the common [`MammalData`].
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Cat::create(ps, "Luna", "gray", "russian_blue", true, false, false);
    ///
    /// let mammals = ps.get_all_mammals();
    /// let mammal_data = mammals[0].mammal_data();
    /// assert_eq!(mammal_data.hair_color, "gray");
    /// ```
    pub fn mammal_data(&self) -> &MammalData {
        match self {
            Mammal::Dog(d) => &d.mammal,
            Mammal::Cat(c) => &c.mammal,
        }
    }

    /// Returns the sound this mammal makes.
    ///
    /// This demonstrates polymorphic behavior through pattern matching.
    /// Returns a static string literal for zero-cost abstraction.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Barky", "white", "husky", true, 12.0, 8);
    /// let (ps, _) = Cat::create(ps, "Meowster", "black", "bombay", true, true, false);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert_eq!(mammals[0].make_sound(), "Woof!");
    /// assert_eq!(mammals[1].make_sound(), "Meow!");
    /// ```
    pub fn make_sound(&self) -> &'static str {
        match self {
            Mammal::Dog(_) => "Woof!",
            Mammal::Cat(_) => "Meow!",
        }
    }

    /// Returns the pet's name.
    ///
    /// Convenience method to access the name without pattern matching.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Spot", "spotted", "dalmatian", true, 11.0, 12);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert_eq!(mammals[0].name(), "Spot");
    /// ```
    pub fn name(&self) -> &str {
        &self.pet_data().name
    }

    /// Returns the pet's UUID.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, dog_id) = Dog::create(ps, "Fido", "brown", "mutt", true, 9.0, 5);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert_eq!(mammals[0].uuid(), dog_id);
    /// ```
    pub fn uuid(&self) -> &str {
        &self.pet_data().uuid
    }

    /// Returns the mammal's hair color.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Cat::create(ps, "Shadow", "black", "bombay", true, false, true);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert_eq!(mammals[0].hair_color(), "black");
    /// ```
    pub fn hair_color(&self) -> &str {
        &self.mammal_data().hair_color
    }

    /// Returns `true` if this is a [`Mammal::Dog`].
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Rover", "white", "poodle", true, 7.0, 20);
    /// let (ps, _) = Cat::create(ps, "Felix", "orange", "tabby", true, true, false);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert!(mammals[0].is_dog());
    /// assert!(!mammals[1].is_dog());
    /// ```
    pub fn is_dog(&self) -> bool {
        matches!(self, Mammal::Dog(_))
    }

    /// Returns `true` if this is a [`Mammal::Cat`].
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Cat::create(ps, "Garfield", "orange", "tabby", true, false, false);
    ///
    /// let mammals = ps.get_all_mammals();
    /// assert!(mammals[0].is_cat());
    /// ```
    pub fn is_cat(&self) -> bool {
        matches!(self, Mammal::Cat(_))
    }

    /// Attempts to downcast to a [`Dog`] reference.
    ///
    /// Returns `Some(&Dog)` if this is a dog, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Dog::create(ps, "Lucky", "brown", "retriever", true, 13.0, 18);
    ///
    /// let mammals = ps.get_all_mammals();
    /// if let Some(dog) = mammals[0].as_dog() {
    ///     assert_eq!(dog.dog_specific.num_commands_known, 18);
    /// }
    /// ```
    pub fn as_dog(&self) -> Option<&Dog> {
        match self {
            Mammal::Dog(dog) => Some(dog),
            _ => None,
        }
    }

    /// Attempts to downcast to a [`Cat`] reference.
    ///
    /// Returns `Some(&Cat)` if this is a cat, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use ecs_inheritance_patterns::pet_module::*;
    ///
    /// let ps = PetState::new();
    /// let (ps, _) = Cat::create(ps, "Smokey", "gray", "chartreux", true, true, false);
    ///
    /// let mammals = ps.get_all_mammals();
    /// if let Some(cat) = mammals[0].as_cat() {
    ///     assert!(cat.cat_specific.declawed);
    /// }
    /// ```
    pub fn as_cat(&self) -> Option<&Cat> {
        match self {
            Mammal::Cat(cat) => Some(cat),
            _ => None,
        }
    }
}

// ============================================================================
// Display Implementations for User-Friendly Output
// ============================================================================

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dog({}, {}, breed: {}, commands: {})",
            self.pet.name,
            self.mammal.hair_color,
            self.mammal.breed,
            self.dog_specific.num_commands_known
        )
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cat({}, {}, keyboard sitter: {})",
            self.pet.name, self.mammal.hair_color, self.cat_specific.sits_on_keyboard
        )
    }
}

impl fmt::Display for Mammal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mammal::Dog(dog) => write!(f, "{}", dog),
            Mammal::Cat(cat) => write!(f, "{}", cat),
        }
    }
}

// ============================================================================
// Constructor Implementation
// ============================================================================

impl Dog {
    /// Creates a new dog and adds it to the PetState
    /// Returns the updated PetState and the UUID of the created dog
    pub fn create(
        mut ps: PetState,
        name: impl Into<String>,
        hair_color: impl Into<String>,
        breed: impl Into<String>,
        has_hair: bool,
        tail_length: f64,
        num_commands_known: i32,
    ) -> (PetState, String) {
        let id = ps.add_dog(
            name.into(),
            hair_color.into(),
            breed.into(),
            has_hair,
            tail_length,
            num_commands_known,
        );
        (ps, id)
    }
}

impl Cat {
    /// Creates a new cat and adds it to the PetState
    /// Returns the updated PetState and the UUID of the created cat
    pub fn create(
        mut ps: PetState,
        name: impl Into<String>,
        hair_color: impl Into<String>,
        breed: impl Into<String>,
        has_hair: bool,
        declawed: bool,
        sits_on_keyboard: bool,
    ) -> (PetState, String) {
        let id = ps.add_cat(
            name.into(),
            hair_color.into(),
            breed.into(),
            has_hair,
            declawed,
            sits_on_keyboard,
        );
        (ps, id)
    }
}

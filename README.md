# Modeling Fowler's Inheritance Patterns in Rust Using ECS

## Overview

This project explores an implementation of **Martin Fowler's inheritance mapping patterns**
(specifically **Class Table Inheritance**) using **Rust** and the **Entity-Component-System (ECS)** pattern.
It addresses the practical considerations that arise when teaching database-to-object mapping patterns
in a language without traditional object-oriented inheritance.

### Background

**The Challenge:**

When teaching Fowler's inheritance patterns in a database course using Rust, several questions emerge:

> "How can students understand class table inheritance patterns when Rust doesn't have inheritance?
> What are the idiomatic approaches to problems typically solved with inheritance in OOP languages?"

**Key Considerations:**
1. How to handle shared fields across similar types without inheritance
2. How to create type-safe polymorphic collections like `Vec<Mammal>`
3. Whether to expose implementation details or maintain abstraction layers

### Feedback Welcome

This implementation represents an exploration of modeling inheritance patterns in Rust. **Feedback is greatly appreciated** — whether you're an instructor, student, Rust practitioner, or professional developer, your insights would be valuable for refining this approach. Please feel free to open an issue or discussion on GitHub sharing your thoughts on:

- The architectural decisions made
- Alternative approaches that might be clearer for teaching
- Improvements to the code or documentation
- Your experience using this in educational contexts
- Real-world applications of similar patterns in production systems

Your feedback helps improve not just this project, but the broader conversation about teaching traditional OOP patterns in modern languages and their practical application in professional settings.

## Implementation Approach

This implementation explores how **Rust's type system offers alternative approaches** through:

1. **Composition** - Building complex types from simpler ones
2. **Enums (sum types)** - Enabling polymorphic collections
3. **Traits** - Defining shared behavior when needed
4. **ECS pattern** - Providing a natural mapping to Class Table Inheritance

## Architecture

```
┌─────────────────────────────────────────┐
│  Public API (Dog, Cat, Mammal enum)     │
│  - Composition structs                  │
│  - Business logic methods               │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  pet_module (domain layer)              │
│  - Type definitions                     │
│  - Constructors                         │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  pet_state module (hidden)              │
│  - ECS World                            │
│  - Components (PetComponent, etc.)      │
│  - Query methods (get_all_dogs, etc.)   │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  Conceptual: Database Tables            │
│  - Pet table (id, name)                 │
│  - Mammal table (pet_id, hair_color...) │
│  - Dog table (mammal_id, tail_length...)│
│  - Cat table (mammal_id, declawed...)   │
└─────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Composition Approach

Rather than flattening all fields into `Dog` and `Cat`, this implementation uses composition:

```rust
pub struct Dog {
    pub pet: PetData,
    pub mammal: MammalData,
    pub dog_specific: DogData,
}
```

**Observations:**
- Adding a field to `MammalData` automatically affects both `Dog` and `Cat`
- The structure mirrors the database table relationships
- Field access requires an extra level (`dog.mammal.hair_color` vs `dog.hair_color`)
- This explicitness can be valuable for understanding the underlying structure

### 2. Enum-Based Polymorphism

For polymorphic collections, this implementation uses enums rather than trait objects:

```rust
pub enum Mammal {
    Dog(Dog),
    Cat(Cat),
}
```

**Characteristics:**
- Enables type-safe `Vec<Mammal>` collections
- Zero-cost abstraction (no heap allocation, no vtable)
- Exhaustive pattern matching at compile time
- Conceptually maps to Single Table Inheritance with a discriminant column

### 3. Hidden ECS Implementation

The ECS components remain private within `pet_state`, while the public API exposes `Dog`, `Cat`, and `Mammal`.

**Rationale:**
- Maintains separation between domain model and persistence strategy
- Demonstrates clear architectural boundaries
- Provides flexibility for future enhancements (caching, lazy loading, etc.)

## Mapping to Class Table Inheritance

### The Pattern (from Fowler)

**Class Table Inheritance:** Each class in the inheritance hierarchy has its own database table. Subclass tables have foreign keys to their superclass tables.

```
Pet Table        Mammal Table       Dog Table
---------        ------------       ---------
id (PK)          id (PK)           id (PK)
name             pet_id (FK)       mammal_id (FK)
                 hair_color        tail_length
                 breed             num_commands
                 has_hair
```

### The ECS Mapping

In ECS, each **component type** represents a **table**:

- `PetComponent` → `Pet` table
- `MammalComponent` → `Mammal` table
- `DogComponent` → `Dog` table

Each **entity** represents a **row** with implicit foreign key relationships.

The `.join()` operation in ECS is equivalent to SQL:

```sql
SELECT * FROM Pet
INNER JOIN Mammal ON Pet.id = Mammal.pet_id
INNER JOIN Dog ON Mammal.id = Dog.mammal_id
```

## Comparative Analysis

### Contrasting with Traditional OOP Approaches

| Aspect                  | OOP (Java/C++)                      | This Implementation         |
|-------------------------|-------------------------------------|-----------------------------|
| Polymorphic Collections | `ArrayList<Mammal>` via inheritance | `Vec<Mammal>` via enums     |
| Database Mapping        | Class table inheritance             | ECS components → tables     |
| Type Safety             | Runtime type checks (instanceof)    | Compile-time exhaustiveness |
| Performance             | Virtual dispatch (vtable)           | Match statement (inlined)   |
| Field Reuse             | Inheritance                         | Composition                 |

### Observations

1. **Different approaches, analogous outcomes:** Both inheritance and composition address data reuse
2. **Type system differences:** Rust leverages sum types where OOP uses subtyping
3. **Performance characteristics:** This approach avoids runtime overhead
4. **Pattern portability:** Architectural patterns can be expressed across paradigms

## Requirements

- **Rust 1.90 or later** (Rust 2024 edition)
- Cargo (comes with Rust)

## Running the Project

```bash
# Build the project
cargo build

# Run the demonstration
cargo run

# Build optimized release version
cargo build --release
```

### Modern Rust Features

This project uses **Rust 2024 edition** with modern features:

- **`impl Into<String>`** - Clean API that accepts both `String` and `&str`
- **Enhanced derives** - `PartialEq`, `Eq`, `Hash` for better type system integration
- **Optimized release profile** - LTO, single codegen unit, stripped binaries
- **Zero-cost abstractions** - Enum-based polymorphism with no runtime overhead

### Expected Output

The program demonstrates:
1. Creating dogs and cats
2. Querying by specific type (all dogs, all cats)
3. **Polymorphic queries** (all mammals via `Vec<Mammal>`)
4. Filtered queries (all blond mammals)
5. Composition structure access
6. Pattern matching with exhaustive checking

## Code Structure

```
ecs_inheritance_patterns/
├── Cargo.toml                      # Project manifest (Rust 2024)
└── src/
    ├── lib.rs                     # Library crate root
    │                              #  - Public API exports
    │                              #  - Crate documentation
    ├── main.rs                    # Demonstration program
    │                              #  - Binary example usage
    ├── pet_module.rs              # Public domain API
    │                              #  - Domain model
    │                              #  - Dog, Cat, Mammal
    └── pet_module/
        └── pet_state.rs           # Hidden implementation
                                   #  - Gateway pattern
                                   #  - ECS components
```

### Why Multi-File Module Structure? ⭐

The project uses **separate files** (`pet_module.rs` + `pet_module/pet_state.rs`) to demonstrate the **Gateway Pattern**:

- **`pet_module.rs`** Public domain API
  - What users interact with: `Dog`, `Cat`, `Mammal`
  - Domain model layer

- **`pet_module/pet_state.rs`** Private data access
  - Hidden ECS implementation
  - Gateway/Repository layer

**Educational Benefits:**
- Shows layered architecture explicitly
- Demonstrates Gateway pattern from Fowler
- Matches professional Rust projects (Tokio, Serde, Diesel)
- Prepares for scalability

## Alternative Approaches

### Trait Objects (`Box<dyn Trait>`)

Rust also supports dynamic dispatch through trait objects:

```rust
pub trait Animal {
    fn make_sound(&self) -> String;
}

// Enables Vec<Box<dyn Animal>>
```

**Trade-offs to consider:**
- Trait objects provide runtime polymorphism (useful for plugins, extensibility)
- They incur heap allocation and vtable lookup costs
- Enums offer compile-time exhaustiveness and zero-cost abstraction
- The choice depends on whether the set of types is known at compile time

## Extending the Project

### Adding New Animal Types

To add a new mammal type (e.g., Horse):

1. **Add component** in `pet_state.rs`:
   ```rust
   #[derive(Component, Debug, Clone)]
   struct HorseComponent {
       speed: f64,
   }
   ```

2. **Add data struct** in `pet_module/mod.rs`:
   ```rust
   pub struct HorseData {
       pub speed: f64,
   }

   pub struct Horse {
       pub pet: PetData,
       pub mammal: MammalData,
       pub horse_specific: HorseData,
   }
   ```

3. **Add enum variant**:
   ```rust
   pub enum Mammal {
       Dog(Dog),
       Cat(Cat),
       Horse(Horse),  // Compiler will now require this in all matches!
   }
   ```

4. **Implement queries** in `pet_state.rs`

The compiler's exhaustiveness checking will require handling the new `Horse` variant in all pattern matches.

### Adding Different Hierarchies

To explore **multiple inheritance hierarchies** (e.g., Reptiles):

1. Create `ReptileData` struct
2. Create `Turtle` and `Snake` structs using composition
3. Create `Reptile` enum
4. Create `Animal` enum that encompasses both mammals and reptiles

This could demonstrate how to model multiple class hierarchies without traditional multiple inheritance.

## Discussion Points

### Conceptual Considerations

1. **Rust's design philosophy:**
   - Favors composition over inheritance
   - Avoids certain OOP complexity (fragile base class problem)
   - Provides zero-cost abstractions by default
   - Uses traits for behavior sharing

2. **Mapping to database patterns:**
   - Class Table Inheritance ↔ ECS components
   - Single Table Inheritance ↔ Enum with discriminant
   - Concrete Table Inheritance ↔ Separate queries per type

3. **Trade-offs in this approach:**
   - Explicitness vs. implicit inheritance
   - Compile-time guarantees vs. runtime flexibility
   - Performance characteristics vs. convenience

### Suggested Exercises

1. Extend the hierarchy (add Horse, Rabbit, etc.)
2. Implement a `Reptile` hierarchy
3. Add query methods (e.g., "Get all pets with names starting with 'S'")
4. Explore lazy loading patterns using UUIDs
5. Implement update operations for mutable state
6. Benchmark enum matching vs. trait object dispatch

## References

- **Martin Fowler - Patterns of Enterprise Application Architecture**
  - Chapter on Object-Relational Structural Patterns
  - Class Table Inheritance pattern
  - [Book on Martin Fowler's website](https://martinfowler.com/books/eaa.html)

- **Luca Palmieri - Zero To Production In Rust**
  - Best practices for building production-ready applications
  - Domain modeling and type-driven design
  - [Book website](https://zero2prod.com)
  - [GitHub repository](https://github.com/LukeMathWalker/zero-to-production)

- **Rust Book**
  - [Chapter 17: Object-Oriented Programming Features](https://doc.rust-lang.org/book/ch17-00-oop.html)
  - [Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)

- **Specs ECS Documentation**
  - [Specs on docs.rs](https://docs.rs/specs/latest/specs/)
  - [Specs GitHub Repository](https://github.com/amethyst/specs)

## Credits

**Original Question:** ShipsMerlin (SWEN400 Large Scale Architectures)

**Implementation:** Response to concerns about modeling inheritance patterns in Rust, specifically addressing field duplication and type-safe polymorphic collections.

## Final Thoughts

This implementation explores how Rust's type system can express traditional OOP database patterns through alternative mechanisms:

1. Architectural patterns can be expressed across different programming paradigms
2. Composition provides a viable alternative to inheritance for field reuse
3. Type systems can offer compile-time guarantees without runtime overhead
4. The relationship between database structures and in-memory representations remains clear

The ECS pattern provides a concrete foundation that maps naturally to Class Table Inheritance while introducing concepts from entity-component architectures used in various high-performance domains.

## License

This is an educational project. Feel free to use and adapt for teaching purposes.

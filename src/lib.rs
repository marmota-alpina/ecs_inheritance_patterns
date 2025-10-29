//! ECS-based implementation of Fowler's Class Table Inheritance pattern.
//!
//! This library demonstrates how to model traditional OOP inheritance patterns
//! in Rust using composition and enums, with Entity-Component-System (ECS)
//! as the underlying storage mechanism.
//!
//! # Overview
//!
//! The library provides:
//! - **Composition-based data structures** ([`Dog`], [`Cat`]) instead of inheritance
//! - **Enum-based polymorphism** ([`Mammal`]) for type-safe collections
//! - **Hidden ECS implementation** through the Gateway pattern
//! - **Zero-cost abstractions** with compile-time type safety
//!
//! # Example
//!
//! ```
//! use ecs_inheritance_patterns::pet_module::*;
//!
//! // Create data access layer
//! let ps = PetState::new();
//!
//! // Add pets using composition
//! let (ps, _) = Dog::create(ps, "Buddy", "golden", "retriever", true, 12.0, 25);
//! let (ps, _) = Cat::create(ps, "Whiskers", "gray", "tabby", true, false, true);
//!
//! // Query by specific type
//! let dogs = ps.get_all_dogs();
//! assert_eq!(dogs.len(), 1);
//!
//! // Query polymorphically
//! let mammals: Vec<Mammal> = ps.get_all_mammals();
//! assert_eq!(mammals.len(), 2);
//!
//! // Access common fields
//! for mammal in mammals {
//!     println!("{} says {}", mammal.name(), mammal.make_sound());
//! }
//! ```
//!
//! # Architecture
//!
//! The crate is organized into layers:
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Public API (pet_module)         │
//! │  - Dog, Cat, Mammal              │
//! │  - PetState (query methods)      │
//! └──────────────────────────────────┘
//!              ↓
//! ┌──────────────────────────────────┐
//! │  Hidden Implementation           │
//! │  - ECS Components                │
//! │  - Gateway/Repository layer      │
//! └──────────────────────────────────┘
//! ```
//!
//! [`Dog`]: pet_module::Dog
//! [`Cat`]: pet_module::Cat
//! [`Mammal`]: pet_module::Mammal

pub mod pet_module;

pub use pet_module::{Cat, Dog, Mammal, PetState};

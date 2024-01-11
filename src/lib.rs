//! This crate contains a set of macros that can be used to generate strongly-typed fields for
//! structs. The macros implement the [newtype] pattern, which allows the compiler to enforce type
//! safety while still making it easy to convert the fields to and from their underlying
//! representation.
//!
//! # Example
//!
//! ```rust
//! use typed_fields::number;
//!
//! // Define a new type that is backed by an `i64`
//! number!(UserId);
//!
//! // Create a new `UserId` from an `i64`
//! let id = UserId::new(42);
//!
//! // Common traits like `Display` are automatically implemented for the type
//! println!("User ID: {}", id);
//! ```
//!
//! [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html

// Code in this library should never panic, which is why we are denying the use of both `expect` and
// `unwrap`. Instead, functions must return a `Result` that can be handled by the caller.
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
// All public items in this library must have documentation.
#![warn(missing_docs)]

mod number;

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

use proc_macro::TokenStream;

mod name;
mod number;
#[cfg(feature = "secrecy")]
mod secret;

/// Generate a new type for a string
///
/// The `name!` macro generates a new type that is backed by a `String`. The new type implements
/// common traits like `Display` and `From<&str>` and `From<String>`. The inner value can be
/// accessed using the `get` method.
///
/// # Example
///
/// ```
/// use typed_fields::name;
///
/// // Define a new type that is backed by a `String`
/// name!(Login);
///
/// // Create a new `UserId` from a `&str`
/// let id = Login::new("jdno");
///
/// // Common traits like `Display` are automatically implemented for the type
/// println!("Login: {}", id);
/// ```
#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    name::name_impl(input)
}

/// Generate a new type for a number
///
/// The `number!` macro generates a new type that is backed by an `i64`. The new type implements
/// common traits like `Display` and `From<i64>`. The inner value can be accessed using the `get`
/// method.
///
/// # Example
///
/// ```
/// use typed_fields::number;
///
/// // Define a new type that is backed by an `i64`
/// number!(UserId);
///
/// // Create a new `UserId` from an `i64`
/// let id = UserId::new(42);
///
/// // Common traits like `Display` are automatically implemented for the type
/// println!("User ID: {}", id);
/// ```
#[proc_macro]
pub fn number(input: TokenStream) -> TokenStream {
    number::number_impl(input)
}

/// Generate a new type for a secret
///
/// The `secret!` macro generates a new type for secrets such as passwords and API tokens. The type
/// uses the [`secrecy`](https://crates.io/crates/secrecy) crate internally to prevent accidentally
/// leaking the inner value in debug or log statements.
///
/// The new type implements common traits like `Display` and `From<&str>` and `From<String>`. The
/// inner value can be revealed using the `expose` method.
///
/// # Example
///
/// ```rust
/// use typed_fields::secret;
///
/// secret!(ApiToken);
///
/// let token: ApiToken = "super-secret-api-token".into();
/// let header = format!("Authorization: Bearer {}", token.expose());
/// ```
#[cfg(feature = "secrecy")]
#[proc_macro]
pub fn secret(input: TokenStream) -> TokenStream {
    secret::secret_impl(input)
}

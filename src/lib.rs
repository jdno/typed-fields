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
#[cfg(feature = "secret")]
mod secret;
#[cfg(feature = "ulid")]
mod ulid;
#[cfg(feature = "url")]
mod url;
#[cfg(feature = "uuid")]
mod uuid;

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
#[cfg(feature = "secret")]
#[proc_macro]
pub fn secret(input: TokenStream) -> TokenStream {
    secret::secret_impl(input)
}

/// Generate a new type for a ULID
///
/// The `ulid!` macro generates a new type that is backed by a `Ulid` from the [`ulid`] crate. The
/// new type implements common traits like `Display` and `From<&str>` and `From<String>`. The inner
/// value can be accessed using the `get` method.
///
/// # Example
///
/// ```rust
/// use typed_fields::ulid;
///
/// ulid!(UserId);
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let id: UserId = "01ARZ3NDEKTSV4RRFFQ69G5FAV".try_into()?;
///     # Ok(())
///     // Do something with the URL...
/// }
/// ```
///
/// [`ulid`]: https://crates.io/crates/ulid
#[cfg(feature = "ulid")]
#[proc_macro]
pub fn ulid(input: TokenStream) -> TokenStream {
    ulid::ulid_impl(input)
}

/// Generate a new type for a URL
///
/// The `url!` macro generates a new type that is backed by a `Url` from the [`url`] crate. The new
/// type implements common traits like `Display` and `TryFrom<&str>` and `TryFrom<String>`. The
/// inner value can be accessed using the `get` method.
///
/// # Example
///
/// ```rust
/// use typed_fields::url;
///
/// url!(BackendUrl);
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let url: BackendUrl = "https://api.example.com".try_into()?;
///     # Ok(())
///     // Do something with the URL...
/// }
/// ```
///
/// [`url`]: https://crates.io/crates/url
#[cfg(feature = "url")]
#[proc_macro]
pub fn url(input: TokenStream) -> TokenStream {
    url::url_impl(input)
}

/// Generate a new type for a UUID
///
/// The `uuid!` macro generates a new type that is backed by a `Uuid` from the [`uuid`] crate. The
/// new type implements common traits like `Display` and `TryFrom<&str>` and `TryFrom<String>`. The
/// inner value can be accessed using the `get` method.
///
/// # Example
///
/// ```rust
/// use typed_fields::uuid;
///
/// uuid!(UserId);
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let id: UserId = "67e55044-10b1-426f-9247-bb680e5fe0c8".try_into()?;
///     # Ok(())
///     // Do something with the URL...
/// }
/// ```
///
/// [`uuid`]: https://crates.io/crates/uuid
#[cfg(feature = "uuid")]
#[proc_macro]
pub fn uuid(input: TokenStream) -> TokenStream {
    uuid::uuid_impl(input)
}

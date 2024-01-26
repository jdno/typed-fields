# 🦀 `typed-fields`

Do you like strongly-typed structs?

`typed-fields` is a collection of macros that generate types following the
[newtype] pattern. The following types are currently supported:

- `name!` - a string-based type
- `number!` - a number-based type
- `secret!` - a type for secrets (requires the `secret` feature)

## Example

The following example showcases the `number!` macro, which generates a new type
that is backed by an `i64`.

```rust
use typed_fields::number;

// Define a new type that is backed by an `i64`
number!(UserId);

fn main() {
    // Create a new `UserId` from an `i64`
    let id = UserId::new(42);

    // Common traits like `Display` are automatically implemented for the type
    println!("User ID: {}", id);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html

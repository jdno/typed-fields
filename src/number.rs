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
#[macro_export]
macro_rules! number {
    (
        $(#[$meta:meta])*
        $id:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct $id(i64);

        impl $id {
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            pub fn get(&self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i64> for $id {
            fn from(id: i64) -> $id {
                $id(id)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    number!(TestId);

    #[test]
    fn get() {
        let id = TestId::new(42);

        assert_eq!(42, id.get());
    }

    #[test]
    fn trait_display() {
        let id = TestId::new(42);

        assert_eq!("42", id.to_string());
    }

    #[test]
    fn trait_from_i64() {
        let _id: TestId = 42.into();
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TestId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TestId>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<TestId>();
    }
}

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
#[macro_export]
macro_rules! name {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        pub struct $name(String);

        impl $name {
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into())
            }

            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            fn from(string: &str) -> $name {
                $name::new(string)
            }
        }

        impl From<String> for $name {
            fn from(string: String) -> $name {
                $name::new(string)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    name!(TestName);

    #[test]
    fn get() {
        let name = TestName::new("test");

        assert_eq!("test", name.get());
    }

    #[test]
    fn trait_display() {
        let name = TestName::new("test");

        assert_eq!("test", name.to_string());
    }

    #[test]
    fn trait_from_string() {
        let _name: TestName = String::from("test").into();
    }

    #[test]
    fn trait_from_str() {
        let _name: TestName = "test".into();
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<TestName>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<TestName>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<TestName>();
    }
}

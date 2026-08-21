// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

#[cfg(feature = "secret")]
use typed_fields::secret;

#[cfg(feature = "secret")]
secret!(
    /// A doc comment for the test secret
    #[derive(serde::Deserialize)]
    TestSecret
);

#[cfg(feature = "secret")]
#[test]
fn deserialize_returns_secret() {
    let json = r#""test""#;

    let config: TestSecret = serde_json::from_str(json).unwrap();

    assert_eq!("test", config.expose());
}

#[cfg(feature = "secret")]
#[test]
fn display_returns_redacted_value() {
    let secret = TestSecret::new("test");

    assert_eq!("[REDACTED]", secret.to_string());
}

#[cfg(feature = "secret")]
#[test]
fn expose_returns_secret() {
    let secret = TestSecret::new("test");

    assert_eq!("test", secret.expose());
}

#[cfg(feature = "secret")]
#[test]
fn from_str_returns_secret() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secret")]
#[test]
fn from_string_returns_secret() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secret")]
#[test]
fn implements_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestSecret>();
}

#[cfg(feature = "secret")]
#[test]
fn implements_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestSecret>();
}

#[cfg(feature = "secret")]
#[test]
fn implements_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestSecret>();
}

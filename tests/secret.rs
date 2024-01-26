#[cfg(feature = "secrecy")]
use typed_fields::secret;

#[cfg(feature = "secrecy")]
secret!(TestSecret);

#[cfg(feature = "secrecy")]
#[test]
fn expose() {
    let secret = TestSecret::new("test");

    assert_eq!("test", secret.expose());
}

#[cfg(all(feature = "secrecy", feature = "serde"))]
#[test]
fn trait_deserialize() {
    let json = r#""test""#;

    let config: TestSecret = serde_json::from_str(json).unwrap();

    assert_eq!("test", config.expose());
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_display() {
    let secret = TestSecret::new("test");

    assert_eq!("[REDACTED]", secret.to_string());
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_from_str() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_from_string() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestSecret>();
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestSecret>();
}

#[cfg(feature = "secrecy")]
#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestSecret>();
}

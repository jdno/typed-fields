// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use typed_fields::number;

number!(
    /// A doc comment for the test id
    #[derive(serde::Deserialize, serde::Serialize)]
    TestId
);

number!(
    /// A `u64` backed number type
    TestU64, u64
);

#[test]
fn deserialize_returns_number() {
    let json = "42";

    let id: TestId = serde_json::from_str(json).unwrap();

    assert_eq!(42, id.get());
}

#[test]
fn display_returns_inner_value() {
    let id = TestId::new(42);

    assert_eq!("42", id.to_string());
}

#[test]
fn from_i64_returns_number() {
    let id: TestId = 42.into();

    assert_eq!(42, id.get());
}

#[test]
fn get_returns_inner_value() {
    let id = TestId::new(42);

    assert_eq!(42, id.get());
}

#[test]
fn get_with_u64_backing_type_returns_value() {
    let id = TestU64::new(42u64);
    assert_eq!(42u64, id.get());
}

#[test]
fn implements_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestId>();
}

#[test]
fn implements_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestId>();
}

#[test]
fn implements_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestId>();
}

#[test]
fn serialize_returns_json() {
    let id = TestId::new(42);

    let json = serde_json::to_string(&id).unwrap();

    assert_eq!("42", json);
}

// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use typed_fields::name;

name!(
    /// A doc comment for the test name
    #[derive(serde::Deserialize, serde::Serialize)]
    TestName
);

#[test]
fn deserialize_returns_name() {
    let json = r#""test""#;

    let name: TestName = serde_json::from_str(json).unwrap();

    assert_eq!("test", name.get());
}

#[test]
fn display_returns_inner_value() {
    let name = TestName::new("test");

    assert_eq!("test", name.to_string());
}

#[test]
fn from_str_returns_name() {
    let name: TestName = "test".into();

    assert_eq!("test", name.get());
}

#[test]
fn from_string_returns_name() {
    let name: TestName = String::from("test").into();

    assert_eq!("test", name.get());
}

#[test]
fn get_returns_inner_value() {
    let name = TestName::new("test");

    assert_eq!("test", name.get());
}

#[test]
fn implements_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestName>();
}

#[test]
fn implements_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestName>();
}

#[test]
fn implements_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestName>();
}

#[test]
fn serialize_returns_json() {
    let name = TestName::new("test");

    let json = serde_json::to_string(&name).unwrap();

    assert_eq!(r#""test""#, json);
}

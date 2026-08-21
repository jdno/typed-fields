// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use std::path::Path;

use typed_fields::path;

path!(
    /// A doc comment for the test path
    #[derive(serde::Deserialize, serde::Serialize)]
    TestPath
);

#[test]
fn deserialize_returns_path() {
    let json = r#""test""#;

    let path: TestPath = serde_json::from_str(json).unwrap();

    assert_eq!(Path::new("test"), path.get());
}

#[test]
fn display_returns_inner_value() {
    let path = TestPath::new("test".into());

    assert_eq!("test", path.to_string());
}

#[test]
fn from_str_returns_path() {
    let path: TestPath = "test".into();

    assert_eq!(Path::new("test"), path.get());
}

#[test]
fn from_string_returns_path() {
    let path: TestPath = String::from("test").into();

    assert_eq!(Path::new("test"), path.get());
}

#[test]
fn get_returns_inner_value() {
    let path = TestPath::new("test".into());

    assert_eq!(Path::new("test"), path.get());
}

#[test]
fn implements_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestPath>();
}

#[test]
fn implements_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestPath>();
}

#[test]
fn implements_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestPath>();
}

#[test]
fn serialize_returns_json() {
    let name: TestPath = "test".into();

    let json = serde_json::to_string(&name).unwrap();

    assert_eq!(r#""test""#, json);
}

// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

#[cfg(feature = "url")]
use std::convert::TryInto;

#[cfg(feature = "url")]
use url::Url;

#[cfg(feature = "url")]
use typed_fields::url;

#[cfg(feature = "url")]
url!(
    /// A doc comment for the test URL
    #[derive(serde::Deserialize, serde::Serialize)]
    TestUrl
);

#[cfg(feature = "url")]
#[test]
fn deserialize_returns_url() {
    let json = r#""postgres://localhost:5432/postgres""#;

    let url: TestUrl = serde_json::from_str(json).unwrap();

    assert_eq!("postgres://localhost:5432/postgres", url.to_string());
}

#[cfg(feature = "url")]
#[test]
fn display_returns_inner_value() {
    let url = TestUrl::new(Url::parse("https://example.com").unwrap());

    assert_eq!("https://example.com/", url.to_string());
}

#[cfg(feature = "url")]
#[test]
fn get_returns_inner_value() {
    let input = Url::parse("postgres://localhost:5432/postgres").unwrap();

    let url = TestUrl::new(input.clone());

    assert_eq!(&input, url.get());
}

#[cfg(feature = "url")]
#[test]
fn implements_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestUrl>();
}

#[cfg(feature = "url")]
#[test]
fn implements_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestUrl>();
}

#[cfg(feature = "url")]
#[test]
fn implements_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestUrl>();
}

#[cfg(feature = "url")]
#[test]
fn serialize_returns_json() {
    let url = TestUrl::new(Url::parse("https://example.com").unwrap());

    let json = serde_json::to_string(&url).unwrap();

    assert_eq!(r#""https://example.com/""#, json);
}

#[cfg(feature = "url")]
#[test]
fn try_from_str_returns_url() {
    let url: TestUrl = "https://example.com/".try_into().unwrap();

    assert_eq!("https://example.com/", url.to_string());
}

#[cfg(feature = "url")]
#[test]
fn try_from_str_with_invalid_input_returns_error() {
    let url = TestUrl::try_from("test");

    assert!(url.is_err());
}

#[cfg(feature = "url")]
#[test]
fn try_from_string_returns_url() {
    let url: TestUrl = String::from("postgres://user:password@locahost:5432/postgres")
        .try_into()
        .unwrap();

    assert_eq!(
        "postgres://user:password@locahost:5432/postgres",
        url.to_string()
    );
}

#[cfg(feature = "url")]
#[test]
fn try_from_string_with_invalid_input_returns_error() {
    let url = TestUrl::try_from(String::from("test"));

    assert!(url.is_err());
}

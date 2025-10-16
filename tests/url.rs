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
    TestUrl
);

#[cfg(feature = "url")]
#[test]
fn get() {
    let input = Url::parse("postgres://localhost:5432/postgres").unwrap();

    let url = TestUrl::new(input.clone());

    assert_eq!(&input, url.get());
}

#[cfg(all(feature = "url", feature = "sea-orm"))]
#[test]
fn compiles_in_sea_orm_model() {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, DeriveEntityModel)]
    #[sea_orm(table_name = "cake")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        url: TestUrl,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(all(feature = "serde", feature = "url"))]
#[test]
fn trait_deserialize() {
    let json = r#""postgres://localhost:5432/postgres""#;

    let url: TestUrl = serde_json::from_str(json).unwrap();

    assert_eq!("postgres://localhost:5432/postgres", url.to_string());
}

#[cfg(feature = "url")]
#[test]
fn trait_display() {
    let url = TestUrl::new(Url::parse("https://example.com").unwrap());

    assert_eq!("https://example.com/", url.to_string());
}

#[cfg(all(feature = "serde", feature = "url"))]
#[test]
fn trait_serialize() {
    let url = TestUrl::new(Url::parse("https://example.com").unwrap());

    let json = serde_json::to_string(&url).unwrap();

    assert_eq!(r#""https://example.com/""#, json);
}

#[cfg(feature = "url")]
#[test]
fn trait_try_from_str() {
    let _url: TestUrl = "https://example.com/".try_into().unwrap();
}

#[cfg(feature = "url")]
#[test]
fn trait_try_from_str_with_random_string() {
    let url = TestUrl::try_from("test");

    assert!(url.is_err());
}

#[cfg(feature = "url")]
#[test]
fn trait_try_from_string() {
    let _url: TestUrl = String::from("postgres://user:password@locahost:5432/postgres")
        .try_into()
        .unwrap();
}

#[cfg(feature = "url")]
#[test]
fn trait_try_from_string_with_random_string() {
    let url = TestUrl::try_from(String::from("test"));

    assert!(url.is_err());
}

#[cfg(feature = "url")]
#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestUrl>();
}

#[cfg(feature = "url")]
#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestUrl>();
}

#[cfg(feature = "url")]
#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestUrl>();
}

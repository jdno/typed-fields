// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

#[cfg(feature = "ulid")]
use std::convert::TryInto;
#[cfg(feature = "ulid")]
use typed_fields::ulid;
#[cfg(feature = "ulid")]
use ulid::Ulid;

#[cfg(feature = "ulid")]
ulid!(
    /// A doc comment for the test ULID
    TestUlid
);

#[cfg(feature = "ulid")]
#[test]
fn get() {
    let input = Ulid::new();

    let ulid = TestUlid::new(input);

    assert_eq!(input, *ulid.get());
}

#[cfg(all(feature = "ulid", feature = "sea-orm"))]
#[test]
fn compiles_in_sea_orm_model() {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, DeriveEntityModel)]
    #[sea_orm(table_name = "cake")]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        ulid: TestUlid,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(all(feature = "serde", feature = "ulid"))]
#[test]
fn trait_deserialize() {
    let json = r#""01ARZ3NDEKTSV4RRFFQ69G5FAV""#;

    let ulid: TestUlid = serde_json::from_str(json).unwrap();

    assert_eq!("01ARZ3NDEKTSV4RRFFQ69G5FAV", ulid.to_string());
}

#[cfg(feature = "ulid")]
#[test]
fn trait_display() {
    let ulid = TestUlid::new(Ulid::from_string("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap());

    assert_eq!("01ARZ3NDEKTSV4RRFFQ69G5FAV", ulid.to_string());
}

#[cfg(all(feature = "serde", feature = "ulid"))]
#[test]
fn trait_serialize() {
    let ulid = TestUlid::new(Ulid::from_string("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap());

    let json = serde_json::to_string(&ulid).unwrap();

    assert_eq!(r#""01ARZ3NDEKTSV4RRFFQ69G5FAV""#, json);
}

#[cfg(feature = "ulid")]
#[test]
fn trait_try_from_str() {
    let _ulid: TestUlid = "01ARZ3NDEKTSV4RRFFQ69G5FAV".try_into().unwrap();
}

#[cfg(feature = "ulid")]
#[test]
fn trait_try_from_str_with_random_string() {
    let ulid = TestUlid::try_from("test");

    assert!(ulid.is_err());
}

#[cfg(feature = "ulid")]
#[test]
fn trait_try_from_string() {
    let _ulid: TestUlid = String::from("01ARZ3NDEKTSV4RRFFQ69G5FAV")
        .try_into()
        .unwrap();
}

#[cfg(feature = "ulid")]
#[test]
fn trait_try_from_string_with_random_string() {
    let ulid = TestUlid::try_from(String::from("test"));

    assert!(ulid.is_err());
}

#[cfg(feature = "ulid")]
#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestUlid>();
}

#[cfg(feature = "ulid")]
#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestUlid>();
}

#[cfg(feature = "ulid")]
#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestUlid>();
}

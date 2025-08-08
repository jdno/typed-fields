// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use typed_fields::number;

number!(
    /// A doc comment for the test id
    TestId
);

#[test]
fn get() {
    let id = TestId::new(42);

    assert_eq!(42, id.get());
}

#[cfg(feature = "sea-orm")]
#[test]
fn compiles_in_sea_orm_model() {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "cake")]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        number: TestId,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(feature = "serde")]
#[test]
fn trait_deserialize() {
    let json = "42";

    let id: TestId = serde_json::from_str(json).unwrap();

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

#[cfg(feature = "serde")]
#[test]
fn trait_serialize() {
    let id = TestId::new(42);

    let json = serde_json::to_string(&id).unwrap();

    assert_eq!("42", json);
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

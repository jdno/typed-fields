// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use typed_fields::name;

name!(
    /// A doc comment for the test name
    TestName
);

#[test]
fn get() {
    let name = TestName::new("test");

    assert_eq!("test", name.get());
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
        name: TestName,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(feature = "serde")]
#[test]
fn trait_deserialize() {
    let json = r#""test""#;

    let name: TestName = serde_json::from_str(json).unwrap();

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

#[cfg(feature = "serde")]
#[test]
fn trait_serialize() {
    let name = TestName::new("test");

    let json = serde_json::to_string(&name).unwrap();

    assert_eq!(r#""test""#, json);
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

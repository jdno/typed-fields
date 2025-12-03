// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

use std::path::Path;

use typed_fields::path;

path!(
    /// A doc comment for the test path
    TestPath
);

#[test]
fn get() {
    let path = TestPath::new("test".into());

    assert_eq!(Path::new("test"), path.get());
}

#[cfg(feature = "sea-orm")]
#[test]
fn compiles_in_sea_orm_model() {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "cake")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        path: TestPath,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(feature = "serde")]
#[test]
fn trait_deserialize() {
    let json = r#""test""#;

    let path: TestPath = serde_json::from_str(json).unwrap();

    assert_eq!(Path::new("test"), path.get());
}

#[test]
fn trait_display() {
    let path = TestPath::new("test".into());

    assert_eq!("test", path.to_string());
}

#[test]
fn trait_from_string() {
    let _path: TestPath = String::from("test").into();
}

#[test]
fn trait_from_str() {
    let _path: TestPath = "test".into();
}

#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestPath>();
}

#[cfg(feature = "serde")]
#[test]
fn trait_serialize() {
    let name: TestPath = "test".into();

    let json = serde_json::to_string(&name).unwrap();

    assert_eq!(r#""test""#, json);
}

#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestPath>();
}

#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestPath>();
}

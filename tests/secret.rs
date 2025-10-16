// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

#[cfg(feature = "secret")]
use typed_fields::secret;

#[cfg(feature = "secret")]
secret!(
    /// A doc comment for the test secret
    TestSecret
);

#[cfg(feature = "secret")]
#[test]
fn expose() {
    let secret = TestSecret::new("test");

    assert_eq!("test", secret.expose());
}

#[cfg(all(feature = "secret", feature = "sea-orm"))]
#[test]
fn compiles_in_sea_orm_model() {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, DeriveEntityModel)]
    #[sea_orm(table_name = "cake")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        id: i32,
        secret: TestSecret,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(all(feature = "secret", feature = "serde"))]
#[test]
fn trait_deserialize() {
    let json = r#""test""#;

    let config: TestSecret = serde_json::from_str(json).unwrap();

    assert_eq!("test", config.expose());
}

#[cfg(feature = "secret")]
#[test]
fn trait_display() {
    let secret = TestSecret::new("test");

    assert_eq!("[REDACTED]", secret.to_string());
}

#[cfg(feature = "secret")]
#[test]
fn trait_from_str() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secret")]
#[test]
fn trait_from_string() {
    let _secret: TestSecret = "test".into();
}

#[cfg(feature = "secret")]
#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestSecret>();
}

#[cfg(feature = "secret")]
#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestSecret>();
}

#[cfg(feature = "secret")]
#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestSecret>();
}

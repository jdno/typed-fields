// TODO: Debug this warning, fix its cause, and remove this directive.
#![allow(non_local_definitions)]

#[cfg(feature = "uuid")]
use std::convert::TryInto;

#[cfg(feature = "uuid")]
use typed_fields::uuid;
#[cfg(feature = "uuid")]
use uuid::uuid as uuid_v4;

#[cfg(feature = "uuid")]
uuid!(TestUuid);

#[cfg(feature = "uuid")]
#[test]
fn get() {
    let input = uuid_v4!("67e55044-10b1-426f-9247-bb680e5fe0c8");

    let uuid = TestUuid::new(input);

    assert_eq!(input, *uuid.get());
}

#[cfg(all(feature = "serde", feature = "uuid"))]
#[test]
fn trait_deserialize() {
    let json = r#""67e55044-10b1-426f-9247-bb680e5fe0c8""#;

    let uuid: TestUuid = serde_json::from_str(json).unwrap();

    assert_eq!("67e55044-10b1-426f-9247-bb680e5fe0c8", uuid.to_string());
}

#[cfg(feature = "uuid")]
#[test]
fn trait_display() {
    let uuid = TestUuid::new(uuid_v4!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

    assert_eq!("67e55044-10b1-426f-9247-bb680e5fe0c8", uuid.to_string());
}

#[cfg(all(feature = "serde", feature = "uuid"))]
#[test]
fn trait_serialize() {
    let uuid = TestUuid::new(uuid_v4!("67e55044-10b1-426f-9247-bb680e5fe0c8"));

    let json = serde_json::to_string(&uuid).unwrap();

    assert_eq!(r#""67e55044-10b1-426f-9247-bb680e5fe0c8""#, json);
}

#[cfg(feature = "uuid")]
#[test]
fn trait_try_from_str() {
    let _uuid: TestUuid = "67e55044-10b1-426f-9247-bb680e5fe0c8".try_into().unwrap();
}

#[cfg(feature = "uuid")]
#[test]
fn trait_try_from_str_with_random_string() {
    let uuid = TestUuid::try_from("test");

    assert!(uuid.is_err());
}

#[cfg(feature = "uuid")]
#[test]
fn trait_try_from_string() {
    let _uuid: TestUuid = String::from("67e55044-10b1-426f-9247-bb680e5fe0c8")
        .try_into()
        .unwrap();
}

#[cfg(feature = "uuid")]
#[test]
fn trait_try_from_string_with_random_string() {
    let uuid = TestUuid::try_from(String::from("test"));

    assert!(uuid.is_err());
}

#[cfg(feature = "uuid")]
#[test]
fn trait_send() {
    fn assert_send<T: Send>() {}
    assert_send::<TestUuid>();
}

#[cfg(feature = "uuid")]
#[test]
fn trait_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<TestUuid>();
}

#[cfg(feature = "uuid")]
#[test]
fn trait_unpin() {
    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<TestUuid>();
}

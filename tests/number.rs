use typed_fields::number;

number!(TestId);

#[test]
fn get() {
    let id = TestId::new(42);

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

use mockall::{automock, predicate::*};

pub struct Storage {}

#[automock]
impl Storage {
    pub fn get<Model: 'static + Default>(&self) -> Model {
        Model::default()
    }

    pub fn save<Model: 'static + Default>(&self, _object: Model) -> Model {
        Model::default()
    }
}

#[test]
fn test_get_same_types() {
    let mut storage = MockStorage::default();
    storage.expect_get::<i32>().return_const(0);
    storage.expect_get::<i32>().return_const(0);

    assert_eq!(storage.get::<i32>(), 0);
    assert_eq!(storage.get::<i32>(), 0);
}

#[test]
fn test_get_different_types() {
    let mut storage = MockStorage::default();
    storage.expect_get::<i32>().return_const(0);
    storage.expect_get::<String>().return_const(String::default());

    assert_eq!(storage.get::<i32>(), 0);
    assert_eq!(storage.get::<String>(), String::default());
}

#[test]
fn test_save_same_types() {
    let mut storage = MockStorage::default();
    storage.expect_save::<i32>().return_const(0);
    storage.expect_save::<i32>().return_const(0);

    assert_eq!(storage.save::<i32>(0), 0);
    assert_eq!(storage.save::<i32>(0), 0);
}

#[test]
fn test_save_different_types() {
    let mut storage = MockStorage::default();
    storage.expect_save::<i32>().return_const(0);
    storage.expect_save::<String>().return_const(String::default());

    assert_eq!(storage.save::<i32>(0), 0);
    assert_eq!(storage.save::<String>(String::default()), String::default());
}

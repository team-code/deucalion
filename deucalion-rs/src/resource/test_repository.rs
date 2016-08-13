
use resource::repository::*;
use error::DeucalionError;
use std::collections::HashMap;

struct StringRepo {
    strings: HashMap<i32, Rc<String>>,
}

impl Repository for StringRepo {
    type T = String;
    type I = i32;
    type E = DeucalionError;

    fn new() -> Self {
        StringRepo { strings: HashMap::new() }
    }

    fn is_available(&self, id: i32) -> bool {
        self.strings.contains_key(&id)
    }

    fn load_object(&mut self, id: i32) -> Result<(), DeucalionError> {
        self.strings.insert(id, Rc::new(id.to_string()));
        Ok(())
    }

    fn get_object(&mut self, id: i32) -> Result<Rc<String>, DeucalionError> {
        if self.is_available(id) {
            Ok(self.get_object_if_available(id).unwrap())
        } else {
            try!(self.load_object(id));
            Ok(self.get_object_if_available(id).unwrap())
        }
    }

    fn get_object_if_available(&self, id: i32) -> Option<Rc<String>> {
        if self.is_available(id) {
            Some(self.strings.get(&id).unwrap().clone())
        } else {
            None
        }
    }
}

#[test]
fn test_repo_lookup() {
    let repo = StringRepo::new();

    assert!(!repo.is_available(1));
}

#[test]
fn test_repo_retrieve() {
    let mut repo = StringRepo::new();

    repo.load_object(1);
    repo.load_object(1337);

    assert_eq!(*repo.get_object_if_available(1).unwrap(), String::from("1"));
    assert_eq!(*repo.get_object_if_available(1337).unwrap(),
               String::from("1337"));
}

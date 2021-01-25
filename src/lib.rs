use std::ops::Deref;

#[derive(Debug)]
struct MaybeOwned<T> {
    shared: bool,
    value: T,
}

struct Shared<T> {
    value: T,
}

impl<T: Clone> MaybeOwned<T> {
    fn new(value: T) -> Self {
        Self {
            shared: false,
            value,
        }
    }

    fn publish(&mut self) -> Shared<T> {
        self.shared = true;
        Shared {
            value: self.value.clone(),
        }
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_the_thing() {
        let item = "42".to_string();
        let mut owned = MaybeOwned::new(item);
        let shared = owned.publish();
        assert_eq!(&**shared, "42");
    }

    #[test]
    fn test_rc() {
        let item = "42".to_string();
        let owned = Rc::new(item);
        let shared = Rc::clone(&owned);
        assert_eq!(&**shared, "42");
    }
}

use std::{any::Any, hash::{Hash, Hasher}};

use crate::utils::unwrap_or_error::UnwrapOrError;

// UNEXPECTED: Unwraps throughout the file are guaranteed to succeed because
// the code will only attempt to downcast to the same type that was used to
// initialize the structure.

pub struct HashEq {
    value: Box<dyn Any>,
    hash: fn(&Box<dyn Any>, &mut dyn Hasher),
    eq: fn(&Box<dyn Any>, &Box<dyn Any>) -> bool
}

impl Hash for HashEq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.hash)(&self.value, state)
    }
}

impl PartialEq for HashEq {
    fn eq(&self, other: &Self) -> bool {
        (self.eq)(&self.value, &other.value)
    }
}

impl Eq for HashEq {}

impl HashEq {
    pub fn new<T: Hash + Eq + 'static>(value: T) -> Self {
        Self {
            value: Box::new(value),
            hash: |value, hasher| {
                let mut hasher = DynHasher(hasher);
                value.type_id().hash(&mut hasher);
                value.downcast_ref::<T>().unwrap_or_unexpected().hash(&mut hasher);
            },
            eq: |a, b| {
                if let Some(b_ref) = b.downcast_ref::<T>() {
                    a.downcast_ref::<T>().unwrap_or_unexpected() == b_ref
                } else {
                    false
                }
            }
        }
    }

    pub fn value(&self) -> &dyn Any {
        &self.value
    }

    pub fn value_owned(self) -> Box<dyn Any> {
        self.value
    }
}

struct DynHasher<'a>(&'a mut dyn Hasher);

impl<'a> Hasher for DynHasher<'a> {
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

pub struct AnyClone {
    value: Box<dyn Any>,
    clone: fn(&Box<dyn Any>) -> Box<dyn Any>
}

impl Clone for AnyClone {
    fn clone(&self) -> Self {
        Self {
            value: (self.clone)(&self.value),
            clone: self.clone
        }
    }
}

impl AnyClone {
    pub fn new<T: Clone + 'static>(value: T) -> Self {
        Self {
            value: Box::new(value),
            clone: |value| Box::new(
                value
                    .downcast_ref::<T>()
                    .unwrap_or_unexpected()
                    .clone()
            )
        }
    }

    pub fn value(&self) -> &dyn Any {
        &*self.value
    }

    pub fn value_owned(self) -> Box<dyn Any> {
        self.value
    }
}

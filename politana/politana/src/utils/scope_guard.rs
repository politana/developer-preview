use crate::utils::unwrap_or_error::UnwrapOrError;

pub struct ScopeGuard<F: FnOnce()>(Option<F>);

impl <F: FnOnce()> ScopeGuard<F> {
    pub fn new(on_drop: F) -> Self { Self(Some(on_drop)) }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(op) = self.0.take() {
            op();
        }
    }
}

pub struct DataGuard<T, F: FnOnce(T)>(Option<T>, Option<F>);

impl <T, F: FnOnce(T)> DataGuard<T, F> {
    pub fn new(data: T, on_drop: F) -> Self { Self(Some(data), Some(on_drop)) }
    // UNEXPECTED: Can only be called before the instance is dropped.
    pub fn value(&self) -> &T { self.0.as_ref().unwrap_or_unexpected() }
}

impl<T, F: FnOnce(T)> Drop for DataGuard<T, F> {
    fn drop(&mut self) {
        if let Some(cleanup) = self.1.take() {
            if let Some(value) = self.0.take() {
                cleanup(value);
            }
        }
    }
}

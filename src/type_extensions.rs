use std::{
    mem,
    sync::{Mutex, PoisonError},
};

pub trait MutexExtensions<T, F> {
    fn replace_with(&self, value: F) -> Result<(), PoisonError<F>>;
}

impl<T: std::default::Default, F: FnOnce(T) -> T> MutexExtensions<T, F> for Mutex<T> {
    /// Calls `f` and replaces the contained value with the result.
    ///
    /// # Errors
    ///
    /// If another user of this mutex panicked while holding the mutex, then
    /// this call will return an error containing the provided `value` instead.
    ///
    /// Partially copied from https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.replace (nightly-only)
    fn replace_with(&self, f: F) -> Result<(), PoisonError<F>> {
        match self.lock() {
            Ok(mut guard) => {
                let mutex_inner = mem::replace(&mut *guard, T::default());
                let _ = mem::replace(&mut *guard, f(mutex_inner));
                Ok(())
            }
            Err(_) => Err(PoisonError::new(f)),
        }
    }
}

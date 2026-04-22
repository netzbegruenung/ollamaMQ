use std::sync::{MutexGuard, PoisonError};
use tracing::error;

pub trait LockExt<'a, T> {
    fn lock_unwrap(self, context: &'static str) -> MutexGuard<'a, T>;
}

impl<'a, T> LockExt<'a, T> for Result<MutexGuard<'a, T>, PoisonError<MutexGuard<'a, T>>> {
    fn lock_unwrap(self, context: &'static str) -> MutexGuard<'a, T> {
        match self {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("Mutex {} poisoned: {}", context, poisoned);
                panic!("Mutex {} poisoned - unrecoverable corruption", context);
            }
        }
    }
}

#[macro_use]
pub mod ordered;

use ordered::After;

use std::sync::{MutexGuard, Mutex};
use std::marker::PhantomData;


/**
  Token used to indicate that a lock on type `T` has been locked.

  A LockToken for `()` can be aquired using the `get_initial_token` function.
*/
pub struct LockedToken<T> {
    _0: PhantomData<T>
}

impl<T> LockedToken<T> {
    unsafe fn get() -> Self {
        Self {
            _0: PhantomData{}
        }
    }
}

/**
  
*/
pub unsafe fn get_initial_token() -> LockedToken<()> {
    LockedToken::get()
}


pub struct OrderedMutex<T> {
    mutex: Mutex<T>,
}

impl<T> OrderedMutex<T> {
    pub fn new(data: T) -> Self {
        OrderedMutex{mutex: Mutex::new(data)}
    }

    pub fn lock<'a, L>(&'a self, _token: &'a mut LockedToken<L>) -> (LockedToken<T>, MutexGuard<'a, T>)
        where
            T: After<L>
    {
        (LockedToken::get(), self.mutex.lock().unwrap())
    }
}


order!(i32, f32);


#[cfg(test)]
mod ordered_tests {
    use super::*;


    #[test]
    fn locking_ordered() {
        let mut initial_token = unsafe{get_initial_token()};

        let mutex1 = OrderedMutex::new(5);
        let mutex2 = OrderedMutex::new(3.3);

        let (mut next_token, data1) = mutex1.lock(&mut initial_token);
        let (_final_token, data2) = mutex2.lock(&mut next_token);

        assert_eq!((*data1, *data2), (5, 3.3));
    }
}

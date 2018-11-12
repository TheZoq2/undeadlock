#[macro_use]
pub mod ordered;

use ordered::After;

use std::sync::{MutexGuard, Mutex};
use std::marker::PhantomData;


/**
  Token used to indicate that a lock on type `T` has been locked.

  A LockToken for `()` can be aquired using the `get_initial_token` function.
*/
pub struct LockedToken<'a, T: 'a> {
    _0: PhantomData<&'a T>
}

impl<'a, T> LockedToken<'a, T> {
    /**
      Returns a `LockedToken` of this type. This should only be called if we
      are sure that any preceding locks that we want to lock have already been locked.
    */
    unsafe fn get() -> Self {
        Self {
            _0: PhantomData{}
        }
    }
}

/**
  Returns a LockedToken<()> which has to be used as a token when locking the first mutex.
  Each thread should only own one and as such, it is recommended to create it when starting
  a new thread.
*/
pub unsafe fn get_initial_token<'a>() -> LockedToken<'a, ()> {
    LockedToken::get()
}


/**
  A mutex which can only be locked if we hold a lock token to a preceding type.
*/
pub struct OrderedMutex<T> {
    mutex: Mutex<T>,
}

impl<T> OrderedMutex<T> {
    /**
      Creates a new OrderedMutex containing `data`
    */
    pub fn new(data: T) -> Self {
        OrderedMutex{mutex: Mutex::new(data)}
    }

    /**
      Locks the mutex and returns a lock token which can be used to lock mutexes
      `After` this one
    */
    pub fn lock<'a, 'b, L>(&'a self, _token: &'b mut LockedToken<L>) -> (LockedToken<'b, T>, MutexGuard<'a, T>)
        where
            T: After<L>,
            'b: 'a
    {
        unsafe {
            (LockedToken::get(), self.mutex.lock().unwrap())
        }
    }
}


order!(f32, i32);


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

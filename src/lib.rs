pub mod ordered;

use ordered::Greater;

use std::sync::{MutexGuard, Mutex};
use std::marker::PhantomData;


pub struct Token<T> {
    _0: PhantomData<T>
}

impl<T> Token<T> {
    fn get() -> Self {
        Self {
            _0: PhantomData{}
        }
    }
}

pub unsafe fn get_initial_token() -> Token<()> {
    Token{_0: PhantomData{}}
}


pub struct OrderedMutex<T> {
    mutex: Mutex<T>,
}

impl<T> OrderedMutex<T> {
    pub fn new(data: T) -> Self {
        OrderedMutex{mutex: Mutex::new(data)}
    }

    pub fn lock<'a, L>(&'a self, _token: &'a mut Token<L>) -> (Token<T>, MutexGuard<'a, T>)
        where
            T: Greater<L>
    {
        (Token::get(), self.mutex.lock().unwrap())
    }
}


impl Greater<()> for i32 {}
impl Greater<i32> for f32 {}
impl Greater<()> for f32 {}


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

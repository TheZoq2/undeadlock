#[macro_use]
extern crate undeadlock;

use std::sync::Arc;

use std::thread;

use undeadlock::{get_initial_token, OrderedMutex};
use undeadlock::ordered::After;

// Unfortunately we have to define newtypes for all our resources that should
// be behind locks since we want to impl After
struct Resource1 (Vec<i32>);
struct Resource2 (Vec<String>);

// Implement Greater such that Resource3 > Resource2 > Resource1 > ()
order!(Resource2, Resource1);


fn main() {
    let r1_lock = Arc::new(OrderedMutex::new(Resource1(vec!())));
    let r2_lock = Arc::new(OrderedMutex::new(Resource2(vec!())));

    // Try playing around with the order that resources are locked
    {
        let (r1_lock, r2_lock) = (r1_lock.clone(), r2_lock.clone());
        thread::spawn(move || {
            // Aquire a lock token for the empty type. Since this is the start of the function
            // we can call it here.
            let mut token = unsafe{get_initial_token()};

            let (mut token, mut r1) = r1_lock.lock(&mut token).unwrap();
            let (mut _token, mut r2) = r2_lock.lock(&mut token).unwrap();

            r1.0.push(5);
            r1.0.push(3);
            r2.0.push(String::from("Hello world"));
        });
    }


    {
        let (r1_lock, r2_lock) = (r1_lock.clone(), r2_lock.clone());
        thread::spawn(move || {
            // Aquire a lock token for the empty type. Since this is the start of the function
            // we can call it here.
            let mut token = unsafe{get_initial_token()};

            let (mut token, mut r1) = r1_lock.lock(&mut token).unwrap();
            let (mut _token, mut r2) = r2_lock.lock(&mut token).unwrap();

            r1.0.push(2);
            r1.0.push(4);
            r2.0.push(String::from("Other thread"));
        });
    }
}

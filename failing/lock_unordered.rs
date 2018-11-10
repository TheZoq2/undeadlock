
extern crate undeadlock;

use undeadlock::{get_initial_token, OrderedMutex};
use undeadlock::ordered::Greater;


struct Lockedi32(i32);
struct Lockedf32(f32);


impl Greater<()> for Lockedi32 {}
impl Greater<Lockedi32> for Lockedf32 {}
impl Greater<()> for Lockedf32 {}


fn main() {
    let mut initial_token = unsafe{get_initial_token()};

    let mutex1 = OrderedMutex::new(Lockedi32(5));
    let mutex2 = OrderedMutex::new(Lockedf32(3.3));

    let (mut next_token, data2) = mutex2.lock(&mut initial_token);
    let (_final_token, data1) = mutex1.lock(&mut next_token);
}

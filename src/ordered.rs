
pub trait Greater<T> {}


struct A {}
struct B {}
struct C {}

impl Greater<B> for A {}
impl Greater<C> for A {}
impl Greater<C> for B {}



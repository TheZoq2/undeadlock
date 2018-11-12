
/**
  Marker trait that is implemented on a type to ensure that that a mutex containing that type
  can only be locked after locks on T have been locked.
*/
pub trait After<T> {}


/**
  Implements the `After` trait for all specified types such that `After<T1..i - 1>` is implemented
  for the ith type
*/
#[macro_export]
macro_rules! order {
    ($first:path) => {
        impl After<()> for $first {}
    };
    ($first:path, $($rest:path),*) => {
        $(
            impl After<$rest> for $first {}
        )*
        order!($($rest),*);
    }
}



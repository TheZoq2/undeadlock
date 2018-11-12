# Undeadlock

Rust provides us with some very nice guarantess about "fearless concurrency". However, while it prevents us from memory issues and race conditions there is one major problem that is left unsolved, deadlocks.

Undeadlock attempts to remove the possibility of deadlocks at compile time to guarantee deadlock free program without runtime costs.


## Background

There are four necessary conditions in which deadlocks can occur 
[1](https://en.wikipedia.org/wiki/Deadlock):

- Mutual exclusion
- Hold and wait or resource holding
- No preemption
- Circular wait

The first and third condition are inherent to mutexes while the second and fourth can
be avoided by always locking mutexes in an agreed-upon order [2]. One way of achieveing that
is to decide an order at runtime, perhaps by looking at the memory address of the data
and locking the lower addresses first.

However this is rust, we have a cool type system and zero cost abstractions for lots of things, why not this as well.


## Implementation

This crate contains two important types: the `After<T>` trait and the `OrderedDeadlock` struct.


### `After<T>`

The `After<T>` trait should be implemented for all types that are contained in mutexes somewhere
in the program. The type can be seen as a `>` operator, meaning that any type `T` which implements
`After<X> > X`. Since it defines an order, it should be irreflexive, antisymetric and
transitive.

It would be nice to express these things in the type system, for example, the transitivity
could be expressed as 

```rust
impl<X, Y, T> After<X> for T
    where Y: After<X>
          T: After<Y>
{}
```

However, as far as I can tell that is not possible with the current type system. As a replacement
the `order!` macro implements `After` for a list of types to achieve the same effect.

Additionally, we assume (and) impl `After<()>` for all types we want to lock.



### `OrderedMutex`

With the `After` trait defined, the `OrderedMutex` struct can be implemented to only allow
locking of a mutex of type `T` if no `X: After<T>` have been locked.

To help with this, the `LockToken<'a, T>` struct is defined. If something holds a mutable
reference to `LockToken<'a, T>` it means that we are free to lock any resource that
implements `After<T>` for a lifetime of `'a`.

When a thread is created, it can aquire a `LockToken<'a ()>` indicating that it is 
able to lock any lock it wants. In the current implementation, a function `get_initial_token()`
is used to aquire such a token. It is marked unsafe as it can be run anywhere in the program
but should only be run when we are sure that no locks are being held. For correct usage
of the library, it should be run as soon as possible after a thread is started.

With a `LockToken<'b T>` in hand, an `OrderedMutex<X>` can be locked using the
`lock(&'b mut token)` function as long as `X: After<T>`.
The lock function returns a `MutexGuard` with a lifetime of `'b` in exchange for a 
`LockToken<'a, T>`. It also returns a `LockToken<'a, X>` that can be used to lock
other mutexes. Since it takes a mutable reference to the lock token, it ensures that
no other locks can use it to lock other locks.



# Future work

In its current state, this library is not very ergonomic, so adding a macro for locking
multiple locks at a time is my first priority. Then I would like to write a sample project
using the library to find bugs and annoyances.

Additionally, while I *think* that the library does what it should I could be wrong. If you
find errors in my reasoning, I would love to hear them.




# References


[1] - https://en.wikipedia.org/wiki/Deadlock
[2] - https://www.researchgate.net/publication/50365788_A_Compile-Time_Deadlock_Detection_Pattern

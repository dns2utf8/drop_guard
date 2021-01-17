//! This crate gives a generic way to add a callback to any dropping value
//!
//! You may use this for debugging values, see the [struct documentation](struct.DropGuard.html) or the [standalone examples](https://github.com/dns2utf8/drop_guard/tree/master/examples).
//!
//! # Example:
//!
//! ```no_run
//! extern crate drop_guard;
//!
//! use drop_guard::DropGuard;
//!
//! use std::thread::{spawn, sleep};
//! use std::time::Duration;
//!
//! fn main() {
//!     // The guard must have a name. `_` will drop it instantly, which would
//!     // lead to unexpected results
//!     let _g = DropGuard::new(spawn(move || {
//!                             sleep(Duration::from_secs(2));
//!                             println!("println! from thread");
//!                         })
//!                         , |join_handle| join_handle.join().unwrap());
//!
//!     println!("Waiting for thread ...");
//! }
//! ```
//!

#![no_std]

use core::ops::{Deref, DerefMut};

/// The DropGuard will remain to `Send` and `Sync` from `T`.
///
/// # Examples
///
/// The `LinkedList<T>` is `Send`.
/// So the `DropGuard` will be too, but it will not be `Sync`:
///
/// ```
/// use drop_guard::DropGuard;
/// use std::collections::LinkedList;
/// use std::thread;
///
/// let list: LinkedList<u32> = LinkedList::new();
///
/// let a_list = DropGuard::new(list, |_| {});
///
/// // Send the guarded list to another thread
/// thread::spawn(move || {
///     assert_eq!(0, a_list.len());
/// }).join();
/// ```
pub struct DropGuard<T, F: FnOnce(T)> {
    data: Option<T>,
    func: Option<F>,
}

impl<T: Sized, F: FnOnce(T)> DropGuard<T, F> {
    /// Creates a new guard taking in your data and a function.
    ///
    /// ```
    /// use drop_guard::DropGuard;
    ///
    /// let s = String::from("a commonString");
    /// let mut s = DropGuard::new(s, |final_string| println!("s became {} at last", final_string));
    ///
    /// // much code and time passes by ...
    /// *s = "a rainbow".to_string();
    ///
    /// // by the end of this function the String will have become a rainbow
    /// ```
    pub fn new(data: T, func: F) -> DropGuard<T, F> {
        DropGuard {
            data: Some(data),
            func: Some(func),
        }
    }

    pub fn into_inner(mut self) -> T {
        self.data.take().expect("the data is here until the drop")
    }
}

/// Use the captured value.
///
/// ```
/// use drop_guard::DropGuard;
///
/// let val = DropGuard::new(42usize, |_| {});
/// assert_eq!(42, *val);
/// ```
impl<T, F: FnOnce(T)> Deref for DropGuard<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data.as_ref().expect("the data is here until the drop")
    }
}

/// Modify the captured value.
///
/// ```
/// use drop_guard::DropGuard;
///
/// let mut val = DropGuard::new(vec![2, 3, 4], |_| {});
/// assert_eq!(3, val.len());
///
/// val.push(5);
/// assert_eq!(4, val.len());
/// ```
impl<T, F: FnOnce(T)> DerefMut for DropGuard<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.as_mut().expect("the data is here until the drop")
    }
}

/// React to dropping the value.
/// In this example we measure the time the value is alive.
///
/// ```
/// use drop_guard::DropGuard;
/// use std::time::Instant;
///
/// let start_time = Instant::now();
/// let val = DropGuard::new(42usize, |_| {
///     let time_alive = start_time.elapsed();
///     println!("value lived for {}ns", time_alive.subsec_nanos())
/// });
/// assert_eq!(42, *val);
/// ```
impl<T, F: FnOnce(T)> Drop for DropGuard<T, F> {
    fn drop(&mut self) {
        if let (Some(data), Some(f)) = (self.data.take(), self.func.take()) {
            f(data);
        }
    }
}

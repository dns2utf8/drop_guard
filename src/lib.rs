//! This crate gives a generic way to add a callback to any dropping value
//! 
//! You may use this for debugging values, see [examples](https://github.com/dns2utf8/drop_guard/tree/master/examples).
//! 
//! # Example:
//! 
//! ```
//! extern crate drop_guard;
//! 
//! use drop_guard::DropGuard;
//! 
//! use std::thread::{spawn, sleep};
//! use std::time::Duration;
//! 
//! fn main() {
//!     // The guard must have a name. `_` will drop it instantly, which woudl lead to 
//!     // unexpected results
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


use std::ops::{Deref, DerefMut, Drop, FnMut};
use std::boxed::Box;

pub struct DropGuard<T, F: FnMut(T)> {
    data: Option<T>,
    func: Box<F>,
}

impl<T: Sized, F: FnMut(T)> DropGuard<T, F> {
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
            func: Box::new(func),
        }
    }
}

impl<T, F: FnMut(T)> Deref for DropGuard<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data.as_ref().expect("the data is here until the drop")
    }
}

impl<T, F: FnMut(T)> DerefMut for DropGuard<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.as_mut().expect("the data is here until the drop")
    }
}

impl<T,F: FnMut(T)> Drop for DropGuard<T, F> {
    fn drop(&mut self) {
        let mut data: Option<T> = None;
        std::mem::swap(&mut data, &mut self.data);
        
        let ref mut f = self.func;
        f(data.expect("the data is here until the drop"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn it_works() {
        let mut i = 0;
        {
            let _ = DropGuard::new(0, |_| i = 42);
        }
        assert_eq!(42, i);
    }

    #[test]
    fn deref() {
        let g = DropGuard::new(5usize, |_| {});
        assert_eq!(5usize, *g);
    }

    #[test]
    fn deref_mut() {
        let mut g = DropGuard::new(5usize, |_| {});
        *g = 12;
        assert_eq!(12usize, *g);
    }

    #[test]
    fn drop_change() {
        let a = Arc::new(AtomicUsize::new(9));
        {
            let _ = DropGuard::new(a.clone()
                                , |i| i.store(42, Ordering::Relaxed));
        }
        assert_eq!(42usize, a.load(Ordering::Relaxed));
    }
}

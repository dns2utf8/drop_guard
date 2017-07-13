use std::ops::{Deref, DerefMut, Drop, FnMut};
use std::boxed::Box;

pub struct DropGuard<T, F: FnMut(&mut T)> {
    data: T,
    func: Box<F>,
}

impl<T: Sized, F: FnMut(&mut T)> DropGuard<T, F> {
    pub fn new(data: T, func: F) -> DropGuard<T, F> {
        DropGuard {
            data,
            func: Box::new(func),
        }
    }
}

impl<T, F: FnMut(&mut T)> Deref for DropGuard<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

impl<T, F: FnMut(&mut T)> DerefMut for DropGuard<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T,F: FnMut(&mut T)> Drop for DropGuard<T, F> {
    fn drop(&mut self) {
        let ref mut f = self.func;
        f(&mut self.data);
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

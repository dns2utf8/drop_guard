extern crate drop_guard;

use drop_guard::*;

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
        let _ = DropGuard::new(a.clone(), |i| i.store(42, Ordering::Relaxed));
    }
    assert_eq!(42usize, a.load(Ordering::Relaxed));
}

#[test]
fn keep_sync_shared_data() {
    fn assert_sync<T: Sync>(_: T) {}
    let g = DropGuard::new(vec![0], |_| {});
    assert_sync(g);
}

#[test]
fn keep_send_shared_data() {
    fn assert_send<T: Send>(_: T) {}
    let g = DropGuard::new(vec![0], |_| {});
    assert_send(g);
}

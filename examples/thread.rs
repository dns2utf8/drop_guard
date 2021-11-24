use drop_guard::guard;

use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let _ = guard(
        spawn(move || {
            sleep(Duration::from_secs(2));
            println!("println! from thread");
        }),
        |join_handle| join_handle.join().unwrap(),
    );

    println!("Waiting for thread ...");
}

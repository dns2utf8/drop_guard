extern crate drop_guard;

use drop_guard::DropGuard;

use std::thread::{spawn, sleep};
use std::time::Duration;

fn main() {
    // The guard must have a name. _ will drop it instantly, which would lead to unexpected results
    let _g = DropGuard::new(spawn(move || {
                            sleep(Duration::from_secs(2));
                            println!("println! from thread");
                        })
                        , |join_handle| join_handle.join().unwrap());
    
    println!("Waiting for thread ...");
}

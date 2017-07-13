extern crate drop_guard;
extern crate threadpool;

use drop_guard::DropGuard;
use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let pool = DropGuard::new(pool, |pool| pool.join());
    
    for i in 0..8 {
        pool.execute(move || print!("{} ", i));
    }
    
    println!("Waiting for threads ...");
}

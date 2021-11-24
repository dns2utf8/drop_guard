extern crate threadpool;

use drop_guard::guard;
use threadpool::ThreadPool;

fn main() {
    a_work_function();
    println!("\nAll done");
}

fn a_work_function() {
    let pool = ThreadPool::new(4);
    let pool = guard(pool, |pool| pool.join());

    for i in 0..8 {
        pool.execute(move || print!("{} ", i));
    }

    println!("Waiting for threads ...");
}

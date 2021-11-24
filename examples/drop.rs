use core::sync::atomic::{AtomicUsize, Ordering};
use drop_guard::guard;

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = vec![];
    for s in (0usize..10).map(|i| format!("{}", i)) {
        v.push(guard(s, |final_string| {
            GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
            if final_string.len() < 5 {
                println!("> still {}", final_string);
            } else {
                println!("> this String became {} at last", final_string);
            }
        }));
    }

    *v[4] = "a rainbow".to_string();

    let back = v.split_off(v.len() / 2);

    // We can drop an object early
    drop(back);

    println!(
        "\n{} Objects dropped already\n",
        GLOBAL_COUNTER.load(Ordering::Relaxed)
    );
}

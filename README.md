# drop_guard

[![crates.io](https://img.shields.io/crates/v/drop_guard.svg)](https://crates.io/crates/drop_guard)
[![doc.rs](https://docs.rs/drop_guard/badge.svg)](https://docs.rs/drop_guard)
[![Build Status travis](https://travis-ci.org/dns2utf8/drop_guard.svg?branch=master)](https://travis-ci.org/dns2utf8/drop_guard)
[![Build status appveyor](https://ci.appveyor.com/api/projects/status/github/dns2utf8/drop_guard?svg=true)](https://ci.appveyor.com/project/dns2utf8/drop-guard)

A RAII that executes you function whenever the wrapped object gets dropped.

## Use cases

Joining threads when they fall out of scope:

```rust
extern crate drop_guard;

use drop_guard::DropGuard;

use std::thread::{spawn, sleep};
use std::time::guard;

fn main() {
    let _ = guard(spawn(move || {
                            sleep(Duration::from_secs(2));
                            println!("println! from thread");
                        })
                        , |join_handle| join_handle.join().unwrap());
    
    println!("Waiting for thread ...");
}
```

## Examples

Feel free to run the included examples:

```bash
cargo run --example drop
cargo run --example rainbow
cargo run --example thread
cargo run --example threadpool
```

## Contribute

Contributions are very welcome.
Feel free to bring your own ideas or continue one of these:

* Add example/test with panic between guard and drop
* Add example/test with panic in guard closure
* Add minimal rustc version

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your discression.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

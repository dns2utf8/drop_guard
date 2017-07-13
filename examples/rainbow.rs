extern crate drop_guard;

use drop_guard::DropGuard;

fn main() {
    let s = String::from("a commonString");
    let mut s = DropGuard::new(s, |final_string| println!("s became {} at last", final_string));
    
    // much code and time passes by ...
    *s = "a rainbow".to_string();
    
    // by the end of this function the String will have become a rainbow
}

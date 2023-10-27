mod my_mod;

use my_mod as one;
use  my_mod::inner_mod as tow;
fn main() {
    println!("a-Z");
    one::print();
    println!("A-z");
    tow::print();
}

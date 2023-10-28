mod my_mod;

use my_mod as one;
use  my_mod::inner_mod as two;
fn main() {
    // a - Z
    one::print_ascii();
    // A - z
    two::print_ascii();
}

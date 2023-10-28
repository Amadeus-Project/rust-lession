mod mod_inner;
mod mod_outer;

fn main() {

    // a - Z
    mod_outer::print_ascii();
    // A - z
    mod_inner::print_ascii();
}

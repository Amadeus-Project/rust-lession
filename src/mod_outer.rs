 use crate::mod_inner::to_ascii;

// a - Z
pub fn print_ascii(){
    println!("a-Z");
    let mut x = 97;
    loop {
        print!("{} ", to_ascii(&x));
        x -= 1;
        if x == 89 {
            break;
        }
    }
    println!();
}

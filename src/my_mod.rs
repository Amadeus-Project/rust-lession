
pub fn to_ascii(i: &i32) -> String {
    match *i {
        x@0..=127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}

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

pub mod inner_mod {
    use crate::my_mod::{to_ascii};
    // A - z
    pub fn print_ascii(){
        println!("A-z");
        for x in 65..= 122{
            print!("{} ", to_ascii(&x));
        }
        println!();
    }


}


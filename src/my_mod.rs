pub fn print_upper(){
    for ch in 'A'..='Z'{
        print!("{ch}");
    }
}

pub fn print_lower(){
    for ch in 'a'..='z'{
        print!("{ch}");
    }
}
pub fn print(){
    print_lower();
    print_upper();
    println!()
}

pub mod inner_mod {
    use crate::my_mod::{print_lower, print_upper};
    pub fn print(){
        print_upper();
        print_lower();
        println!()
    }
}


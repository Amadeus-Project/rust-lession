
// A - z
pub fn print_ascii(){
    println!("A-z");
    for x in 65..= 122{
        print!("{} ", to_ascii(&x));
    }
    println!();
}


pub fn to_ascii(i: &i32) -> String {
    match *i {
        x@0..=127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}


use std::io;
fn main() {

     println!("User Input");
    let mut a = String::new();
    io::stdin().read_line(&mut a);

    actual_length(&mut a);
}

fn actual_length(s: &mut String) {
    println!("Length {}", s.len() )
}

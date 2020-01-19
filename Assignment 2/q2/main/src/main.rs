fn main() {
    
    let mut s = String::from("PAKISTAN");
    
    add_two(&mut s);
    println!(" {}",s);  
}

fn add_two(country: &mut String) {
    country.push_str(" ZINDABAD");

}
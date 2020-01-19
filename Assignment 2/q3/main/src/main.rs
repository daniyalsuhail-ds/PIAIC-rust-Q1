use std::io;
fn main() {

    println!("Enter Number 1");
    let mut a = String::new();
    io::stdin().read_line(&mut a);
    let a:i32 = a.trim().parse().unwrap();

    println!("Enter Number 2");
    let mut b = String::new();
    io::stdin().read_line(&mut b);
    let b:i32 = b.trim().parse().unwrap();

    println!("Enter Number 3");
    let mut c = String::new();
    io::stdin().read_line(&mut c);
    let c:i32 = c.trim().parse().unwrap();

    println!("average = {}", (a+b+c)/3);
}

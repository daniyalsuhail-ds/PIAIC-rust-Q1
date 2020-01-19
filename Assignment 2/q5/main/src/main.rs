use std::io;

fn main() {

    println!("Enter Number");
    let mut number = String::new();
    io::stdin().read_line(&mut number);
    let number:u128 = number.trim().parse().unwrap();
    
    for x in 0..number {

        for y in 0..x+1 {

        print!("*",);
        
        }
        print!("\n", );
    }


}

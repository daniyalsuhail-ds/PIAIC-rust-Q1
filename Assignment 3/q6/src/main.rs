fn main() {
    
    
    let laptop1 = laptops::Lenovo();
    let laptop2 = laptops::Asus();
    let laptop3 = laptops::Dell(3000_series);
    let laptop4 = laptops::Dell(5000_series);
}

#[derive(Debug)]
enum 6_variant {
    000_series, 
    3000_series,
    4000_series,
    5000_series,
    6000_series,

}
#[derive(Debug)]
enum laptops {
    HP,
    Dell (6_variant),
    Asus,
    Lenovo,
}



fn value_in_cents(laptop: laptops) -> u8 {
    match laptop {
       laptops::HP => {
            println!("HP");
        },
     laptops::Asus => {
            println!("Asus");
            
        },
       laptops::Dell(6_variant) => {
            println!("Dell {:?}", 6_variant);
        },
    
    }
}
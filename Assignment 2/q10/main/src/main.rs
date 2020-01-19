use std::io;

#[derive(Debug)]
struct Person {
    name :String,
    age : String,
    country : String,
}


fn main() {

     println!("User Name");
    let mut person_name = String::new();
    io::stdin().read_line(&mut person_name);
    let person_name = person_name.trim().parse().unwrap();

    println!("User Age");
    let mut person_age = String::new();
    io::stdin().read_line(&mut person_age);
     let person_age = person_age.trim().parse().unwrap();

    println!("User Country");
    let mut person_country = String::new();
    io::stdin().read_line(&mut person_country);
     let person_country = person_country.trim().parse().unwrap();


    let person1 = Person {
        name: person_name,
        age: person_age,
        country: person_country,
    };
   
   let xyz = [person1.name, person1.age, person1.country];
   println!("{:#?}", xyz );
}

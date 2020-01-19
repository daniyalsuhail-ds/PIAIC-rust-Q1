#[derive(Debug)]
struct Student{
    name: String,
    email: String,
    phone_no: u32,
    gender: String ,
}

fn main() {

let student1 = Student{
    name: String::from("ALI"),
    email: String::from("ali@gmail.com"),
    phone_no: 12456321,
    gender: String::from("Male"),
};

let student2 = Student{
    name: String::from("Huzaifa"),
    email: String::from("huzaifa@yahoo.com"),
    phone_no: 554326,
    gender: String::from("Male"),
};

         println!("{}", student1.email );
        println!("{:#?}", student2 );

}
fn main() {

    let student_1 = Student::Onsite( Name: String::from("Ali"), RollNo: String::from("asd-123"));
    let student_2 = Student::Online( Name: String::from("Daniyal"), RollNo: String::from("das-123"));

}


#[derive(Debug)]
enum Student {
    Onsite {Name:String, RollNo: String},
    Online {Name:String, RollNo: String},
}
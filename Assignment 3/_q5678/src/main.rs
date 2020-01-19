fn main() {
    
    
    let shape_1 = Shapes::Circle(34);
    let shape_2 = Shapes::Triangle(34,67,32);
    let shape_3 = Shapes::Rectangle(34,78,32,43);
    let shape_4 = Shapes::Square(34,24,65,32);

    shape_1.call();
    shape_2.call();
    shape_3.call();
    shape_4.call();



}

#[derive(Debug)]
enum Shapes {
    Circle(i32),
    Triangle(i32, i32 ,i32),
    Rectangle(i32 ,i32 ,i32 ,i32),
    Square(i32, i32, i32, i32),
}

impl Shapes {
    fn call(&self) {
        println!("{:?}",self );
    }
}
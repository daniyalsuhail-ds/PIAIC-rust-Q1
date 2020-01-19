struct Rectangle {
width: u32,
height: u32}


fn main(){
let rect1 = Rectangle{
width: 50,
height: 100
};

 area(rect1.width, rect1.height);
}

fn area(x:u32, y:u32) {
    println!("Area = {}", x*y )
}
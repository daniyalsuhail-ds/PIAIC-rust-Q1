#[derive(Debug)]
struct Triangle {
    length1: u32,
    length2: u32,
    length3: u32,
}

impl Triangle{
    fn sum(&self) -> u32 {
        self.length1 + self.length2 + self.length3
    }

    fn large_side(&self)  {
        if self.length1 > self.length2 && self.length1 > self.length3  {
            println!("length 1 is greater" );
        }

        else if self.length2 > self.length1 && self.length2 > self.length3  {
            println!("length 2 is greater" );
        }

        else if self.length3 > self.length1 && self.length3 > self.length2  {
            println!("length 3 is greater" );
        }
    }
}

fn main() {
    let triangle1 = Triangle {

        length1: 25,
        length2: 80,
        length3: 60, 
    
         };

    println!(
        "The sum of all side of the trianlge is {}",
        triangle1.sum()
    );

    triangle1.large_side();
}
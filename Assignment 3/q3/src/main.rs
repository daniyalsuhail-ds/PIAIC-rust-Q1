fn main() {
   
   let vehicle_1 = Vehicle::trucks(String::from("3123esd"));
   let vehicle_2 = Vehicle::bikes(String::from("uimmuasdas"));
   let vehicle_3 = Vehicle::cars(String::from("asasdasdas"));
}

enum Vehicle {
    trucks (String),
    cars  (String),
    bikes  (String),
}
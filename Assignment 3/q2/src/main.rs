fn main() {

    let corolla = transport {
    kind: Vehicle::cars,
    registration_no: String::from("CF3223"),
};

let bmx = transport {
    kind: Vehicle::bikes,
    registration_no: String::from("ghj23"),
};

let wagon = transport {
    kind: Vehicle::trucks,
    registration_no: String::from("24323"),
};



}

struct transport {
    kind: Vehicle,
    registration_no: String,
}

enum Vehicle {
    trucks,
    cars,
    bikes,
}
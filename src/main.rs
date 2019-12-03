use std::fs;
use math::round;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut total_fuel = 0.0;
    let split = contents.split("\n");

    for s in split{
        let mass: f64 = s.trim().parse().unwrap();
        let mut fuel: f64 = calculate_fuel(mass);
        total_fuel += fuel;

        while fuel != 0.0 {
            fuel = calculate_fuel(fuel);
            total_fuel += fuel;
        }
    }

    println!("Total fuel requirements: {}", total_fuel);
}

fn calculate_fuel(mass: f64) -> f64 {
    let fuel: f64 = round::floor(mass / 3.0, 0) - 2.0;

    if fuel < 0.0 {
        return 0.0
    }
    else {
        return fuel
    }
}
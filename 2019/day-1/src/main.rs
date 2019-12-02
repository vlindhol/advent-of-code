use std::env;
use std::fs;
use std::i64;

fn fuel_requirement(mass: i64, acc: i64) -> i64 {
    println!("Mass! {} {}", mass, acc);
    if mass < 0 {
     return acc;
    }
    let result = mass / 3 - 2;
    if result < 0 {
        return acc;
    }
    return fuel_requirement(result, acc + result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let text_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input: Vec<i64> = text_input.lines().map(|v| v.parse::<i64>().unwrap_or(0)).collect();
    let fuel_requirements: Vec<i64> = input.iter().map(|v| fuel_requirement(*v, 0)).collect();
    let sum: i64 = fuel_requirements.iter().sum();
    println!("Sum: {}", sum);
}
